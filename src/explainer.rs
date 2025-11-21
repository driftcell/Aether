//! Explainer for Aether - Converts symbol-dense code to human-readable format
//!
//! This module provides projectional editing functionality, translating
//! Aether's high-density UTF-8 symbols into readable pseudo-code that
//! humans can understand.

use crate::parser::{AstNode, LiteralValue};

/// Explains an Aether AST in human-readable format
pub struct Explainer {
    indent_level: usize,
}

impl Explainer {
    pub fn new() -> Self {
        Self { indent_level: 0 }
    }

    /// Convert an AST to human-readable explanation
    pub fn explain(&mut self, ast: &[AstNode]) -> String {
        let mut output = String::new();
        
        for node in ast {
            output.push_str(&self.explain_node(node));
            output.push('\n');
        }
        
        output
    }

    fn explain_node(&mut self, node: &AstNode) -> String {
        match node {
            AstNode::Function { name, body } => {
                format!(
                    "{}function {}:\n{}",
                    self.indent(),
                    name,
                    self.with_indent(|e| e.explain_node(body))
                )
            }
            
            AstNode::Sequence(nodes) => {
                let mut output = String::new();
                for (i, n) in nodes.iter().enumerate() {
                    if i > 0 {
                        output.push_str(&format!("\n{}then\n", self.indent()));
                    }
                    output.push_str(&self.explain_node(n));
                }
                output
            }
            
            AstNode::Input => format!("{}read input", self.indent()),
            
            AstNode::Output(value) => {
                format!("{}output {}", self.indent(), self.explain_node(value))
            }
            
            AstNode::Pipe { source, operation } => {
                format!(
                    "{} | {}",
                    self.explain_node(source),
                    self.explain_node(operation)
                )
            }
            
            AstNode::PipeInto { value, variable } => {
                format!(
                    "{} -> store in {}",
                    self.explain_node(value),
                    variable
                )
            }
            
            AstNode::Guard { condition, then_branch } => {
                format!(
                    "{}guard (if {} is null or invalid):\n{}",
                    self.indent(),
                    self.explain_node(condition),
                    self.with_indent(|e| e.explain_node(then_branch))
                )
            }
            
            AstNode::Halt(code) => {
                format!("{}halt with error {}", self.indent(), self.explain_node(code))
            }
            
            AstNode::Persist(value) => {
                format!("{}save to database: {}", self.indent(), self.explain_node(value))
            }
            
            AstNode::JsonParse(value) => {
                format!("parse JSON({})", self.explain_node(value))
            }
            
            AstNode::Variable(name) => name.clone(),
            
            AstNode::Literal(lit) => self.explain_literal(lit),
            
            AstNode::Empty => "null".to_string(),
            
            AstNode::IfThen { condition, then_branch, else_branch } => {
                let mut output = format!(
                    "{}if {}:\n{}",
                    self.indent(),
                    self.explain_node(condition),
                    self.with_indent(|e| e.explain_node(then_branch))
                );
                
                if let Some(else_node) = else_branch {
                    output.push_str(&format!(
                        "\n{}else:\n{}",
                        self.indent(),
                        self.with_indent(|e| e.explain_node(else_node))
                    ));
                }
                output
            }
            
            AstNode::Loop { condition, body } => {
                if let Some(cond) = condition {
                    format!(
                        "{}while {}:\n{}",
                        self.indent(),
                        self.explain_node(cond),
                        self.with_indent(|e| e.explain_node(body))
                    )
                } else {
                    format!(
                        "{}loop forever:\n{}",
                        self.indent(),
                        self.with_indent(|e| e.explain_node(body))
                    )
                }
            }
            
            AstNode::ForEach { variable, collection, body } => {
                format!(
                    "{}for each {} in {}:\n{}",
                    self.indent(),
                    variable,
                    self.explain_node(collection),
                    self.with_indent(|e| e.explain_node(body))
                )
            }
            
            AstNode::Filter { predicate } => {
                format!("filter where {}", self.explain_node(predicate))
            }
            
            AstNode::Reduce { operation, initial } => {
                format!(
                    "reduce with {} starting from {}",
                    self.explain_node(operation),
                    self.explain_node(initial)
                )
            }
            
            AstNode::TryRescue { try_body, rescue_body } => {
                let mut output = format!(
                    "{}try:\n{}",
                    self.indent(),
                    self.with_indent(|e| e.explain_node(try_body))
                );
                
                if let Some(rescue) = rescue_body {
                    output.push_str(&format!(
                        "\n{}on error:\n{}",
                        self.indent(),
                        self.with_indent(|e| e.explain_node(rescue))
                    ));
                }
                output
            }
            
            AstNode::Retry { max_attempts, body } => {
                let attempts_str = if let Some(attempts) = max_attempts {
                    attempts.to_string()
                } else {
                    "unlimited".to_string()
                };
                format!(
                    "{}retry up to {} times:\n{}",
                    self.indent(),
                    attempts_str,
                    self.with_indent(|e| e.explain_node(body))
                )
            }
            
            AstNode::Async { body } => {
                format!(
                    "{}execute asynchronously:\n{}",
                    self.indent(),
                    self.with_indent(|e| e.explain_node(body))
                )
            }
            
            AstNode::Await { expression } => {
                format!("{}wait for {}", self.indent(), self.explain_node(expression))
            }
            
            AstNode::Import { module } => {
                format!("{}import module {}", self.indent(), module)
            }
            
            AstNode::Split { target, delimiter } => {
                if let Some(delim) = delimiter {
                    format!(
                        "split {} by {}",
                        self.explain_node(target),
                        self.explain_node(delim)
                    )
                } else {
                    format!("split {}", self.explain_node(target))
                }
            }
            
            AstNode::Join { elements, separator } => {
                if let Some(sep) = separator {
                    format!(
                        "join {} with separator {}",
                        self.explain_node(elements),
                        self.explain_node(sep)
                    )
                } else {
                    format!("join {}", self.explain_node(elements))
                }
            }
            
            AstNode::RegexMatch { pattern, target } => {
                format!(
                    "match pattern {} against {}",
                    self.explain_node(pattern),
                    self.explain_node(target)
                )
            }
            
            AstNode::Equal { left, right } => {
                format!(
                    "({} == {})",
                    self.explain_node(left),
                    self.explain_node(right)
                )
            }
            
            AstNode::NotEqual { left, right } => {
                format!(
                    "({} != {})",
                    self.explain_node(left),
                    self.explain_node(right)
                )
            }
            
            AstNode::And { left, right } => {
                format!(
                    "({} and {})",
                    self.explain_node(left),
                    self.explain_node(right)
                )
            }
            
            AstNode::Or { left, right } => {
                format!(
                    "({} or {})",
                    self.explain_node(left),
                    self.explain_node(right)
                )
            }
            
            AstNode::Not { operand } => {
                format!(
                    "(not {})",
                    self.explain_node(operand)
                )
            }
            
            AstNode::Comparison { left, operator, right } => {
                let op_str = match operator {
                    crate::parser::ComparisonOp::GreaterThan => ">",
                    crate::parser::ComparisonOp::LessThan => "<",
                };
                format!(
                    "({} {} {})",
                    self.explain_node(left),
                    op_str,
                    self.explain_node(right)
                )
            }
            
            AstNode::Approx { left, right } => {
                format!(
                    "({} ≈ {})",
                    self.explain_node(left),
                    self.explain_node(right)
                )
            }
            
            AstNode::HttpGet { url, headers } => {
                let mut result = format!("{}HTTP GET request to {}", self.indent(), self.explain_node(url));
                if headers.is_some() {
                    result.push_str(" with headers");
                }
                result
            }
            
            AstNode::HttpPost { url, body, headers } => {
                let mut result = format!("{}HTTP POST to {}", self.indent(), self.explain_node(url));
                if let Some(b) = body {
                    result.push_str(&format!(" with body {}", self.explain_node(b)));
                }
                if headers.is_some() {
                    result.push_str(" with headers");
                }
                result
            }
            
            AstNode::HttpPut { url, body, headers } => {
                let mut result = format!("{}HTTP PUT to {}", self.indent(), self.explain_node(url));
                if let Some(b) = body {
                    result.push_str(&format!(" with body {}", self.explain_node(b)));
                }
                if headers.is_some() {
                    result.push_str(" with headers");
                }
                result
            }
            
            AstNode::HttpDelete { url, headers } => {
                let mut result = format!("{}HTTP DELETE request to {}", self.indent(), self.explain_node(url));
                if headers.is_some() {
                    result.push_str(" with headers");
                }
                result
            }
            
            AstNode::Random => "generate random number".to_string(),
            
            AstNode::DateTime => "get current date/time".to_string(),
            
            AstNode::Log { message } => {
                format!("{}log message: {}", self.indent(), self.explain_node(message))
            }
            
            AstNode::Assert { condition } => {
                format!("{}assert {}", self.indent(), self.explain_node(condition))
            }
            
            AstNode::Hash { data } => {
                format!("hash {}", self.explain_node(data))
            }
            
            AstNode::Power { base, exponent } => {
                format!(
                    "{} raised to the power of {}",
                    self.explain_node(base),
                    self.explain_node(exponent)
                )
            }
            
            AstNode::Root { value } => {
                format!("square root of {}", self.explain_node(value))
            }
            
            AstNode::ReadContent { source } => {
                format!("{}read from {}", self.indent(), self.explain_node(source))
            }
            
            AstNode::WriteContent { target, content } => {
                format!(
                    "{}write to {} content: {}",
                    self.indent(),
                    self.explain_node(target),
                    self.explain_node(content)
                )
            }
            
            AstNode::ShellExec { command } => {
                format!(
                    "{}execute shell command: {}",
                    self.indent(),
                    self.explain_node(command)
                )
            }
            
            AstNode::EnvVar { name } => {
                format!("get environment variable {}", self.explain_node(name))
            }
            
            AstNode::Immutable { name, value } => {
                format!(
                    "{}define constant {} = {}",
                    self.indent(),
                    name,
                    self.explain_node(value)
                )
            }
            
            AstNode::Debug => {
                format!("{}enable debug mode", self.indent())
            }
            
            _ => format!("{}[unknown operation]", self.indent()),
        }
    }
    
    fn explain_literal(&self, lit: &LiteralValue) -> String {
        match lit {
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => format!("\"{}\"", s),
        }
    }
    
    fn indent(&self) -> String {
        "  ".repeat(self.indent_level)
    }
    
    fn with_indent<F>(&mut self, f: F) -> String
    where
        F: FnOnce(&mut Self) -> String,
    {
        self.indent_level += 1;
        let result = f(self);
        self.indent_level -= 1;
        result
    }
}

impl Default for Explainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Lexer, Parser};

    #[test]
    fn test_explain_simple_output() {
        let source = r#"📤 "Hello""#;
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut explainer = Explainer::new();
        let explanation = explainer.explain(&ast);
        
        assert!(explanation.contains("output"));
        assert!(explanation.contains("Hello"));
    }
    
    #[test]
    fn test_explain_pipe_into() {
        let source = "10 ▷ x";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut explainer = Explainer::new();
        let explanation = explainer.explain(&ast);
        
        assert!(explanation.contains("store in x"));
    }
    
    #[test]
    fn test_explain_conditional() {
        let source = "10 ▷ x ⨠ ◇(x > 5): 📤\"Large\"";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut explainer = Explainer::new();
        let explanation = explainer.explain(&ast);
        
        assert!(explanation.contains("if"));
        assert!(explanation.contains("output"));
    }
}
