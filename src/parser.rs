//! Parser for building Abstract Syntax Trees from Aether tokens

use crate::error::{AetherError, Result};
use crate::lexer::{Token, TokenType};
use crate::symbols::Symbol;

/// AST Node representing Aether code structure
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// Function definition: name, body
    Function {
        name: String,
        body: Box<AstNode>,
    },

    /// Sequence of operations
    Sequence(Vec<AstNode>),

    /// Input operation
    Input,

    /// Output operation with value
    Output(Box<AstNode>),

    /// Pipe operation: source -> operation
    Pipe {
        source: Box<AstNode>,
        operation: Box<AstNode>,
    },

    /// PipeInto: value -> variable name
    PipeInto {
        value: Box<AstNode>,
        variable: String,
    },

    /// Guard (null check) with alternative
    Guard {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
    },

    /// Halt with error code
    Halt(Box<AstNode>),

    /// Persist operation
    Persist(Box<AstNode>),

    /// JSON parse operation
    JsonParse(Box<AstNode>),

    /// Variable reference
    Variable(String),

    /// Literal value
    Literal(LiteralValue),

    /// Empty/null
    Empty,
    
    // Control Flow & Iteration (v1.1)
    /// Loop: body
    Loop {
        body: Box<AstNode>,
    },
    
    /// ForEach: iterator variable, collection, body
    ForEach {
        variable: String,
        collection: Box<AstNode>,
        body: Box<AstNode>,
    },
    
    /// Filter: predicate
    Filter {
        predicate: Box<AstNode>,
    },
    
    /// Reduce: accumulator, initial value
    Reduce {
        operation: Box<AstNode>,
        initial: Box<AstNode>,
    },
    
    /// Try-Rescue: try_body, rescue_body
    TryRescue {
        try_body: Box<AstNode>,
        rescue_body: Option<Box<AstNode>>,
    },
    
    /// Retry: max_attempts, body
    Retry {
        max_attempts: Option<i32>,
        body: Box<AstNode>,
    },
    
    // Concurrency & Async (v1.1)
    /// Async execution
    Async {
        body: Box<AstNode>,
    },
    
    /// Await result
    Await {
        expression: Box<AstNode>,
    },
    
    /// Thread/Task
    Thread {
        body: Box<AstNode>,
    },
    
    /// Lock/Mutex
    Lock {
        body: Box<AstNode>,
    },
    
    /// Emit/Signal
    Emit {
        event: Box<AstNode>,
    },
    
    /// Watch/Listen
    Watch {
        event: Box<AstNode>,
        handler: Box<AstNode>,
    },
    
    // Data Manipulation (v1.1)
    /// Split/Slice
    Split {
        target: Box<AstNode>,
        delimiter: Option<Box<AstNode>>,
    },
    
    /// Join/Concat
    Join {
        elements: Box<AstNode>,
        separator: Option<Box<AstNode>>,
    },
    
    /// Regex/Match
    RegexMatch {
        pattern: Box<AstNode>,
        target: Box<AstNode>,
    },
    
    /// Equal comparison
    Equal {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    
    /// Not Equal comparison
    NotEqual {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    
    /// Immutable/Const
    Immutable {
        name: String,
        value: Box<AstNode>,
    },
    
    // System & Environment (v1.1)
    /// Import module
    Import {
        module: String,
    },
    
    /// Auth/Token
    Auth {
        token: Box<AstNode>,
    },
    
    /// DateTime
    DateTime,
    
    /// Random
    Random,
    
    /// Log
    Log {
        message: Box<AstNode>,
    },
    
    /// HTTP GET (composed operation)
    HttpGet {
        url: Box<AstNode>,
    },
}

/// Literal values in Aether
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    String(String),
    Number(f64),
}

/// Parser for Aether tokens
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser from tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    /// Parse tokens into an AST
    pub fn parse(&mut self) -> Result<Vec<AstNode>> {
        let mut nodes = Vec::new();

        while !self.is_at_end() {
            let node = self.parse_statement()?;
            nodes.push(node);
        }

        Ok(nodes)
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> Result<AstNode> {
        // Check for function definition
        if self.match_symbol(&Symbol::Function) {
            return self.parse_function();
        }

        // Otherwise, parse an expression
        self.parse_expression()
    }

    /// Parse a function definition
    fn parse_function(&mut self) -> Result<AstNode> {
        // Expect function name (identifier or symbol)
        let name = if let Some(token) = self.peek() {
            let name_str = match &token.token_type {
                TokenType::Symbol(Symbol::Identifier(id)) => id.clone(),
                TokenType::Symbol(Symbol::Register) => "register".to_string(),
                _ => "anonymous".to_string(),
            };
            if name_str != "anonymous" {
                self.advance();
            }
            name_str
        } else {
            "anonymous".to_string()
        };

        // Expect colon
        if !self.match_token_type(&TokenType::Colon) {
            // Allow functions without colon for flexibility
        }

        // Parse function body
        let body = self.parse_expression()?;

        Ok(AstNode::Function {
            name,
            body: Box::new(body),
        })
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> Result<AstNode> {
        self.parse_sequence()
    }

    /// Parse a sequence of operations (separated by ⨠)
    fn parse_sequence(&mut self) -> Result<AstNode> {
        let mut operations = vec![self.parse_pipe()?];

        while self.match_symbol(&Symbol::Sequence) {
            operations.push(self.parse_pipe()?);
        }

        if operations.len() == 1 {
            Ok(operations.into_iter().next().unwrap())
        } else {
            Ok(AstNode::Sequence(operations))
        }
    }

    /// Parse pipe operations
    fn parse_pipe(&mut self) -> Result<AstNode> {
        let mut expr = self.parse_primary()?;

        while self.match_symbol(&Symbol::Pipe) || self.match_symbol(&Symbol::PipeInto) {
            let was_pipe_into = matches!(
                self.tokens.get(self.position - 1).map(|t| &t.token_type),
                Some(TokenType::Symbol(Symbol::PipeInto))
            );

            if was_pipe_into {
                // PipeInto: bind to variable
                let var_name = if let Some(token) = self.peek() {
                    let id_opt = match &token.token_type {
                        TokenType::Symbol(Symbol::Identifier(id)) => Some(id.clone()),
                        _ => None,
                    };
                    
                    if let Some(id) = id_opt {
                        self.advance();
                        id
                    } else {
                        return Err(AetherError::ParserError(
                            "Expected identifier after ▷".to_string(),
                        ));
                    }
                } else {
                    return Err(AetherError::ParserError(
                        "Expected identifier after ▷".to_string(),
                    ));
                };

                expr = AstNode::PipeInto {
                    value: Box::new(expr),
                    variable: var_name,
                };
            } else {
                // Regular pipe
                let operation = self.parse_primary()?;
                expr = AstNode::Pipe {
                    source: Box::new(expr),
                    operation: Box::new(operation),
                };
            }
        }

        Ok(expr)
    }

    /// Parse primary expressions
    fn parse_primary(&mut self) -> Result<AstNode> {
        if let Some(token) = self.peek() {
            match &token.token_type {
                TokenType::Symbol(Symbol::Input) => {
                    self.advance();
                    Ok(AstNode::Input)
                }
                TokenType::Symbol(Symbol::Output) => {
                    self.advance();
                    let value = self.parse_primary()?;
                    Ok(AstNode::Output(Box::new(value)))
                }
                TokenType::Symbol(Symbol::JsonParse) => {
                    self.advance();
                    Ok(AstNode::JsonParse(Box::new(AstNode::Input)))
                }
                TokenType::Symbol(Symbol::Guard) => {
                    self.advance();
                    let then_branch = self.parse_primary()?;
                    Ok(AstNode::Guard {
                        condition: Box::new(AstNode::Empty),
                        then_branch: Box::new(then_branch),
                    })
                }
                TokenType::Symbol(Symbol::Halt) => {
                    self.advance();
                    let error_code = self.parse_primary()?;
                    Ok(AstNode::Halt(Box::new(error_code)))
                }
                TokenType::Symbol(Symbol::Persist) => {
                    self.advance();
                    let value = if !self.is_at_end() && !self.check_symbol(&Symbol::Sequence) {
                        self.parse_primary()?
                    } else {
                        AstNode::Empty
                    };
                    Ok(AstNode::Persist(Box::new(value)))
                }
                TokenType::Symbol(Symbol::Identifier(id)) => {
                    let id = id.clone();
                    self.advance();
                    Ok(AstNode::Variable(id))
                }
                TokenType::Symbol(Symbol::StringLiteral(s)) => {
                    let s = s.clone();
                    self.advance();
                    Ok(AstNode::Literal(LiteralValue::String(s)))
                }
                TokenType::Symbol(Symbol::NumberLiteral(n)) => {
                    let n = *n;
                    self.advance();
                    Ok(AstNode::Literal(LiteralValue::Number(n)))
                }
                TokenType::Symbol(Symbol::Empty) => {
                    self.advance();
                    Ok(AstNode::Empty)
                }
                // Control Flow & Iteration
                TokenType::Symbol(Symbol::Loop) => {
                    self.advance();
                    let body = self.parse_primary()?;
                    Ok(AstNode::Loop {
                        body: Box::new(body),
                    })
                }
                TokenType::Symbol(Symbol::ForEach) => {
                    self.advance();
                    // Expect pattern: ∀(variable): body
                    // For simplicity: read next identifier as variable, then body
                    let variable = if let Some(token) = self.peek() {
                        match &token.token_type {
                            TokenType::Symbol(Symbol::Identifier(id)) => {
                                let id = id.clone();
                                self.advance();
                                id
                            }
                            _ => "it".to_string(),
                        }
                    } else {
                        "it".to_string()
                    };
                    
                    // Skip colon if present
                    self.match_token_type(&TokenType::Colon);
                    
                    let body = self.parse_primary()?;
                    Ok(AstNode::ForEach {
                        variable,
                        collection: Box::new(AstNode::Empty), // Will be set by pipe
                        body: Box::new(body),
                    })
                }
                TokenType::Symbol(Symbol::Filter) => {
                    self.advance();
                    let predicate = self.parse_primary()?;
                    Ok(AstNode::Filter {
                        predicate: Box::new(predicate),
                    })
                }
                TokenType::Symbol(Symbol::Reduce) => {
                    self.advance();
                    let operation = self.parse_primary()?;
                    Ok(AstNode::Reduce {
                        operation: Box::new(operation),
                        initial: Box::new(AstNode::Literal(LiteralValue::Number(0.0))),
                    })
                }
                TokenType::Symbol(Symbol::Try) => {
                    self.advance();
                    let try_body = self.parse_primary()?;
                    Ok(AstNode::TryRescue {
                        try_body: Box::new(try_body),
                        rescue_body: None,
                    })
                }
                TokenType::Symbol(Symbol::Retry) => {
                    self.advance();
                    // Check if next token is a number (retry count)
                    let max_attempts = if let Some(token) = self.peek() {
                        match &token.token_type {
                            TokenType::Symbol(Symbol::NumberLiteral(n)) => {
                                let count = *n as i32;
                                self.advance();
                                Some(count)
                            }
                            _ => None,
                        }
                    } else {
                        None
                    };
                    
                    // Skip colon if present
                    self.match_token_type(&TokenType::Colon);
                    
                    let body = self.parse_primary()?;
                    Ok(AstNode::Retry {
                        max_attempts,
                        body: Box::new(body),
                    })
                }
                // Concurrency & Async
                TokenType::Symbol(Symbol::Async) => {
                    self.advance();
                    let body = self.parse_primary()?;
                    Ok(AstNode::Async {
                        body: Box::new(body),
                    })
                }
                TokenType::Symbol(Symbol::Await) => {
                    self.advance();
                    let expression = self.parse_primary()?;
                    Ok(AstNode::Await {
                        expression: Box::new(expression),
                    })
                }
                TokenType::Symbol(Symbol::Thread) => {
                    self.advance();
                    let body = self.parse_primary()?;
                    Ok(AstNode::Thread {
                        body: Box::new(body),
                    })
                }
                TokenType::Symbol(Symbol::Lock) => {
                    self.advance();
                    let body = self.parse_primary()?;
                    Ok(AstNode::Lock {
                        body: Box::new(body),
                    })
                }
                TokenType::Symbol(Symbol::Emit) => {
                    self.advance();
                    let event = self.parse_primary()?;
                    Ok(AstNode::Emit {
                        event: Box::new(event),
                    })
                }
                TokenType::Symbol(Symbol::Watch) => {
                    self.advance();
                    let event = self.parse_primary()?;
                    // Simplified: expect handler as next expression
                    let handler = if !self.is_at_end() && !self.check_symbol(&Symbol::Sequence) {
                        self.parse_primary()?
                    } else {
                        AstNode::Empty
                    };
                    Ok(AstNode::Watch {
                        event: Box::new(event),
                        handler: Box::new(handler),
                    })
                }
                // Data Manipulation
                TokenType::Symbol(Symbol::Split) => {
                    self.advance();
                    let delimiter = if !self.is_at_end() && !self.check_symbol(&Symbol::Sequence) {
                        Some(Box::new(self.parse_primary()?))
                    } else {
                        None
                    };
                    Ok(AstNode::Split {
                        target: Box::new(AstNode::Empty), // Will be set by pipe
                        delimiter,
                    })
                }
                TokenType::Symbol(Symbol::Join) => {
                    self.advance();
                    let separator = if !self.is_at_end() && !self.check_symbol(&Symbol::Sequence) {
                        Some(Box::new(self.parse_primary()?))
                    } else {
                        None
                    };
                    Ok(AstNode::Join {
                        elements: Box::new(AstNode::Empty), // Will be set by pipe
                        separator,
                    })
                }
                TokenType::Symbol(Symbol::Regex) => {
                    self.advance();
                    let pattern = self.parse_primary()?;
                    Ok(AstNode::RegexMatch {
                        pattern: Box::new(pattern),
                        target: Box::new(AstNode::Empty), // Will be set by pipe
                    })
                }
                TokenType::Symbol(Symbol::Equal) => {
                    self.advance();
                    let right = self.parse_primary()?;
                    Ok(AstNode::Equal {
                        left: Box::new(AstNode::Empty), // Will be set by pipe
                        right: Box::new(right),
                    })
                }
                TokenType::Symbol(Symbol::NotEqual) => {
                    self.advance();
                    let right = self.parse_primary()?;
                    Ok(AstNode::NotEqual {
                        left: Box::new(AstNode::Empty), // Will be set by pipe
                        right: Box::new(right),
                    })
                }
                TokenType::Symbol(Symbol::Immutable) => {
                    self.advance();
                    let name = if let Some(token) = self.peek() {
                        match &token.token_type {
                            TokenType::Symbol(Symbol::Identifier(id)) => {
                                let id = id.clone();
                                self.advance();
                                id
                            }
                            _ => "const".to_string(),
                        }
                    } else {
                        "const".to_string()
                    };
                    let value = self.parse_primary()?;
                    Ok(AstNode::Immutable {
                        name,
                        value: Box::new(value),
                    })
                }
                // System & Environment
                TokenType::Symbol(Symbol::Import) => {
                    self.advance();
                    let module = if let Some(token) = self.peek() {
                        match &token.token_type {
                            TokenType::Symbol(Symbol::Identifier(id)) => {
                                let id = id.clone();
                                self.advance();
                                id
                            }
                            TokenType::Symbol(Symbol::HttpRequest) => {
                                self.advance();
                                "http".to_string()
                            }
                            _ => "module".to_string(),
                        }
                    } else {
                        "module".to_string()
                    };
                    Ok(AstNode::Import { module })
                }
                TokenType::Symbol(Symbol::Auth) => {
                    self.advance();
                    let token = self.parse_primary()?;
                    Ok(AstNode::Auth {
                        token: Box::new(token),
                    })
                }
                TokenType::Symbol(Symbol::DateTime) => {
                    self.advance();
                    Ok(AstNode::DateTime)
                }
                TokenType::Symbol(Symbol::Random) => {
                    self.advance();
                    Ok(AstNode::Random)
                }
                TokenType::Symbol(Symbol::Log) => {
                    self.advance();
                    let message = self.parse_primary()?;
                    Ok(AstNode::Log {
                        message: Box::new(message),
                    })
                }
                TokenType::Symbol(Symbol::HttpRequest) => {
                    self.advance();
                    // Check if next is Input symbol for composed GET operation
                    if self.match_symbol(&Symbol::Input) {
                        let url = self.parse_primary()?;
                        Ok(AstNode::HttpGet {
                            url: Box::new(url),
                        })
                    } else {
                        Ok(AstNode::Variable("http".to_string()))
                    }
                }
                _ => Err(AetherError::ParserError(format!(
                    "Unexpected token: {:?}",
                    token
                ))),
            }
        } else {
            Err(AetherError::UnexpectedEof)
        }
    }

    /// Check if we're at the end
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
            || matches!(
                self.tokens.get(self.position).map(|t| &t.token_type),
                Some(TokenType::Eof)
            )
    }

    /// Peek at current token
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Advance to next token
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.tokens.get(self.position - 1)
    }

    /// Check if current token matches a symbol
    fn check_symbol(&self, symbol: &Symbol) -> bool {
        if let Some(token) = self.peek() {
            matches!(&token.token_type, TokenType::Symbol(s) if s == symbol)
        } else {
            false
        }
    }

    /// Match and consume a symbol
    fn match_symbol(&mut self, symbol: &Symbol) -> bool {
        if self.check_symbol(symbol) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Match and consume a token type
    fn match_token_type(&mut self, token_type: &TokenType) -> bool {
        if let Some(token) = self.peek() {
            if std::mem::discriminant(&token.token_type) == std::mem::discriminant(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_input_output() {
        let mut lexer = Lexer::new("📥 📤200".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 2);
    }

    #[test]
    fn test_parse_function() {
        let mut lexer = Lexer::new("ƒ®: 📥".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Function { name, .. } => assert_eq!(name, "register"),
            _ => panic!("Expected function node"),
        }
    }

    #[test]
    fn test_parse_sequence() {
        let mut lexer = Lexer::new("📥 ⨠ 📤200".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Sequence(ops) => assert_eq!(ops.len(), 2),
            _ => panic!("Expected sequence node"),
        }
    }
    
    #[test]
    fn test_parse_foreach() {
        let mut lexer = Lexer::new("∀u: 📤u".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::ForEach { variable, .. } => assert_eq!(variable, "u"),
            _ => panic!("Expected ForEach node"),
        }
    }
    
    #[test]
    fn test_parse_filter() {
        let mut lexer = Lexer::new("∃ res".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Filter { .. } => {},
            _ => panic!("Expected Filter node"),
        }
    }
    
    #[test]
    fn test_parse_async() {
        let mut lexer = Lexer::new("⚡ 📥".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Async { .. } => {},
            _ => panic!("Expected Async node"),
        }
    }
    
    #[test]
    fn test_parse_retry() {
        let mut lexer = Lexer::new("♻3: 🌐".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Retry { max_attempts, .. } => assert_eq!(*max_attempts, Some(3)),
            _ => panic!("Expected Retry node"),
        }
    }
    
    #[test]
    fn test_parse_try_rescue() {
        let mut lexer = Lexer::new("🛡 📥".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::TryRescue { .. } => {},
            _ => panic!("Expected TryRescue node"),
        }
    }
    
    #[test]
    fn test_parse_import() {
        let mut lexer = Lexer::new("🧩🌐".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Import { module } => assert_eq!(module, "http"),
            _ => panic!("Expected Import node"),
        }
    }
    
    #[test]
    fn test_parse_log() {
        let mut lexer = Lexer::new("🪵 \"message\"".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Log { .. } => {},
            _ => panic!("Expected Log node"),
        }
    }
    
    #[test]
    fn test_parse_split() {
        let mut lexer = Lexer::new("✂ \",\"".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Split { .. } => {},
            _ => panic!("Expected Split node"),
        }
    }
    
    #[test]
    fn test_parse_datetime() {
        let mut lexer = Lexer::new("📅".to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::DateTime => {},
            _ => panic!("Expected DateTime node"),
        }
    }
}
