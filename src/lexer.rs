//! Lexer for tokenizing Aether source code

use crate::error::{AetherError, Result};
use crate::symbols::Symbol;
use unicode_segmentation::UnicodeSegmentation;

/// Token type in the Aether language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Symbol(Symbol),
    Whitespace,
    Newline,
    Colon,
    LeftParen,
    RightParen,
    Dot,
    GreaterThan,
    LessThan,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Eof,
}

/// A token with position information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub position: usize,
    pub length: usize,
}

impl Token {
    pub fn new(token_type: TokenType, position: usize, length: usize) -> Self {
        Token {
            token_type,
            position,
            length,
        }
    }
}

/// Lexer for tokenizing Aether source code
pub struct Lexer {
    position: usize,
    graphemes: Vec<String>,
}

impl Lexer {
    /// Create a new lexer from source code
    pub fn new(input: String) -> Self {
        let graphemes: Vec<String> = input.graphemes(true).map(|s| s.to_string()).collect();
        Lexer {
            position: 0,
            graphemes,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while self.position < self.graphemes.len() {
            let token = self.next_token()?;
            // Skip whitespace and newline tokens
            if matches!(token.token_type, TokenType::Whitespace | TokenType::Newline) {
                continue;
            }
            tokens.push(token);
        }

        tokens.push(Token::new(TokenType::Eof, self.position, 0));
        Ok(tokens)
    }

    /// Get the next token
    fn next_token(&mut self) -> Result<Token> {
        if self.position >= self.graphemes.len() {
            return Ok(Token::new(TokenType::Eof, self.position, 0));
        }

        let start_pos = self.position;
        let current = &self.graphemes[self.position];

        // Handle comments (// to end of line)
        if current == "/" && self.position + 1 < self.graphemes.len() && self.graphemes[self.position + 1] == "/" {
            // Skip to end of line
            while self.position < self.graphemes.len() {
                let ch = &self.graphemes[self.position];
                self.position += 1;
                if ch == "\n" || ch == "\r" {
                    return Ok(Token::new(TokenType::Newline, start_pos, self.position - start_pos));
                }
            }
            return Ok(Token::new(TokenType::Eof, self.position, 0));
        }
        
        // Handle division operator (must be after comment check)
        if current == "/" {
            self.position += 1;
            return Ok(Token::new(TokenType::Symbol(Symbol::Divide), start_pos, 1));
        }
        
        // Handle arithmetic operators that might be confused with other syntax
        if current == "*" {
            self.position += 1;
            return Ok(Token::new(TokenType::Symbol(Symbol::Multiply), start_pos, 1));
        }
        
        if current == "%" {
            self.position += 1;
            return Ok(Token::new(TokenType::Symbol(Symbol::Modulo), start_pos, 1));
        }

        // Handle whitespace
        if current == " " || current == "\t" {
            self.position += 1;
            return Ok(Token::new(TokenType::Whitespace, start_pos, 1));
        }

        // Handle newlines
        if current == "\n" || current == "\r" {
            self.position += 1;
            return Ok(Token::new(TokenType::Newline, start_pos, 1));
        }

        // Handle colon
        if current == ":" {
            self.position += 1;
            return Ok(Token::new(TokenType::Colon, start_pos, 1));
        }

        // Handle parentheses
        if current == "(" {
            self.position += 1;
            return Ok(Token::new(TokenType::LeftParen, start_pos, 1));
        }

        if current == ")" {
            self.position += 1;
            return Ok(Token::new(TokenType::RightParen, start_pos, 1));
        }

        // Handle dot operator
        if current == "." {
            self.position += 1;
            return Ok(Token::new(TokenType::Dot, start_pos, 1));
        }

        // Handle comparison operators
        if current == ">" {
            self.position += 1;
            return Ok(Token::new(TokenType::GreaterThan, start_pos, 1));
        }

        if current == "<" {
            self.position += 1;
            return Ok(Token::new(TokenType::LessThan, start_pos, 1));
        }
        
        // Handle brackets for array/indexing
        if current == "[" {
            self.position += 1;
            return Ok(Token::new(TokenType::LeftBracket, start_pos, 1));
        }
        
        if current == "]" {
            self.position += 1;
            return Ok(Token::new(TokenType::RightBracket, start_pos, 1));
        }
        
        // Handle braces for objects
        if current == "{" {
            self.position += 1;
            return Ok(Token::new(TokenType::LeftBrace, start_pos, 1));
        }
        
        if current == "}" {
            self.position += 1;
            return Ok(Token::new(TokenType::RightBrace, start_pos, 1));
        }
        
        // Handle comma
        if current == "," {
            self.position += 1;
            return Ok(Token::new(TokenType::Comma, start_pos, 1));
        }

        // Handle string literals
        if current == "\"" {
            return self.read_string_literal(start_pos);
        }

        // Handle numbers (including negative numbers)
        if current.chars().next().map_or(false, |c| c.is_numeric()) {
            return self.read_number(start_pos);
        }

        // Try to parse as a known symbol
        if let Some(symbol) = Symbol::from_str(current) {
            self.position += 1;
            return Ok(Token::new(TokenType::Symbol(symbol), start_pos, 1));
        }

        // Handle identifiers (alphanumeric sequences)
        if current.chars().next().map_or(false, |c| c.is_alphanumeric() || c == '_') {
            return self.read_identifier(start_pos);
        }

        // Unknown character
        Err(AetherError::LexerError(format!(
            "Unknown character at position {}: '{}'",
            start_pos, current
        )))
    }

    /// Read a string literal
    fn read_string_literal(&mut self, start_pos: usize) -> Result<Token> {
        self.position += 1; // Skip opening quote
        let mut content = String::new();

        while self.position < self.graphemes.len() {
            let ch = &self.graphemes[self.position];
            if ch == "\"" {
                self.position += 1; // Skip closing quote
                let length = self.position - start_pos;
                return Ok(Token::new(
                    TokenType::Symbol(Symbol::StringLiteral(content)),
                    start_pos,
                    length,
                ));
            }
            content.push_str(ch);
            self.position += 1;
        }

        Err(AetherError::LexerError(
            "Unterminated string literal".to_string(),
        ))
    }

    /// Read a number
    fn read_number(&mut self, start_pos: usize) -> Result<Token> {
        let mut number_str = String::new();

        while self.position < self.graphemes.len() {
            let ch = &self.graphemes[self.position];
            if ch.chars().next().map_or(false, |c| c.is_numeric() || c == '.') {
                number_str.push_str(ch);
                self.position += 1;
            } else {
                break;
            }
        }

        let number = number_str
            .parse::<f64>()
            .map_err(|e| AetherError::LexerError(format!("Invalid number: {}", e)))?;

        let length = self.position - start_pos;
        Ok(Token::new(
            TokenType::Symbol(Symbol::NumberLiteral(number)),
            start_pos,
            length,
        ))
    }

    /// Read an identifier
    fn read_identifier(&mut self, start_pos: usize) -> Result<Token> {
        let mut identifier = String::new();

        while self.position < self.graphemes.len() {
            let ch = &self.graphemes[self.position];
            if ch.chars().next().map_or(false, |c| c.is_alphanumeric() || c == '_') {
                identifier.push_str(ch);
                self.position += 1;
            } else {
                break;
            }
        }

        // Check if identifier is a single-char symbol
        if identifier.len() == 1 {
            if let Some(symbol) = Symbol::from_str(&identifier) {
                let length = self.position - start_pos;
                return Ok(Token::new(TokenType::Symbol(symbol), start_pos, length));
            }
        }

        let length = self.position - start_pos;
        Ok(Token::new(
            TokenType::Symbol(Symbol::Identifier(identifier)),
            start_pos,
            length,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic_symbols() {
        let mut lexer = Lexer::new("ƒ 📥 📤".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4); // 3 symbols + EOF
        assert!(matches!(tokens[0].token_type, TokenType::Symbol(Symbol::Function)));
        assert!(matches!(tokens[1].token_type, TokenType::Symbol(Symbol::Input)));
        assert!(matches!(tokens[2].token_type, TokenType::Symbol(Symbol::Output)));
    }

    #[test]
    fn test_lexer_string_literal() {
        let mut lexer = Lexer::new("\"hello world\"".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // string + EOF
        match &tokens[0].token_type {
            TokenType::Symbol(Symbol::StringLiteral(s)) => assert_eq!(s, "hello world"),
            _ => panic!("Expected string literal"),
        }
    }

    #[test]
    fn test_lexer_number() {
        let mut lexer = Lexer::new("42.5".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // number + EOF
        match &tokens[0].token_type {
            TokenType::Symbol(Symbol::NumberLiteral(n)) => assert_eq!(*n, 42.5),
            _ => panic!("Expected number literal"),
        }
    }

    #[test]
    fn test_lexer_identifier() {
        let mut lexer = Lexer::new("user_id".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // identifier + EOF
        match &tokens[0].token_type {
            TokenType::Symbol(Symbol::Identifier(id)) => assert_eq!(id, "user_id"),
            _ => panic!("Expected identifier"),
        }
    }

    #[test]
    fn test_lexer_complex() {
        let mut lexer = Lexer::new("ƒ®: 📥⇢J".to_string());
        let tokens = lexer.tokenize().unwrap();

        // Should tokenize: ƒ ® : 📥 ⇢ J EOF
        assert!(tokens.len() >= 6);
    }
}
