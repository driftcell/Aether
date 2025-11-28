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
                let source_str = self.explain_node(source);
                let operation_str = self.explain_node(operation);
                
                // If source is the special _pipe variable, simplify output
                if source_str == "_pipe" {
                    operation_str
                } else {
                    format!("{} | {}", source_str, operation_str)
                }
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
                    crate::parser::ComparisonOp::GreaterEqual => ">=",
                    crate::parser::ComparisonOp::LessEqual => "<=",
                };
                format!(
                    "({} {} {})",
                    self.explain_node(left),
                    op_str,
                    self.explain_node(right)
                )
            }
            
            AstNode::Approx { left, right } => {
                let left_str = self.explain_node(left);
                let right_str = self.explain_node(right);
                
                // If left is the special _pipe variable, simplify output
                if left_str == "_pipe" {
                    format!("(piped value) ≈ {}", right_str)
                } else {
                    format!("({} ≈ {})", left_str, right_str)
                }
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
                let base_str = self.explain_node(base);
                let exp_str = self.explain_node(exponent);
                
                // If base is the special _pipe variable, simplify output
                if base_str == "_pipe" {
                    format!("(piped value) raised to the power of {}", exp_str)
                } else {
                    format!("{} raised to the power of {}", base_str, exp_str)
                }
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
            
            // File System (v1.3) - Additional operations not yet fully explained
            AstNode::FileHandle { .. } => format!("{}file handle", self.indent()),
            AstNode::Directory { .. } => format!("{}directory", self.indent()),
            AstNode::PathResolve { .. } => format!("{}path resolution", self.indent()),
            AstNode::AppendContent { .. } => format!("{}append to file", self.indent()),
            AstNode::DeleteFile { .. } => format!("{}delete file", self.indent()),
            AstNode::SetPermission { .. } => format!("{}set file permissions", self.indent()),
            
            // Streams & Buffers (v1.3)
            AstNode::CreateStream { .. } => format!("{}create stream", self.indent()),
            AstNode::CreateBuffer { .. } => format!("{}create buffer", self.indent()),
            AstNode::FlushBuffer { .. } => format!("{}flush buffer", self.indent()),
            AstNode::EndOfFile => format!("{}end of file", self.indent()),
            AstNode::SkipBytes { .. } => format!("{}skip bytes", self.indent()),
            
            // Networking (v1.3)
            AstNode::CreateSocket { .. } => format!("{}create socket", self.indent()),
            AstNode::ListenPort { .. } => format!("{}listen on port", self.indent()),
            AstNode::ConnectRemote { .. } => format!("{}connect to remote", self.indent()),
            AstNode::PortNumber { .. } => format!("{}port number", self.indent()),
            AstNode::CreatePacket { .. } => format!("{}create packet", self.indent()),
            AstNode::Handshake { .. } => format!("{}handshake", self.indent()),
            
            // Process & OS (v1.3)
            AstNode::ProcessCreate { .. } => format!("{}create process", self.indent()),
            AstNode::MemoryAlloc { .. } => format!("{}allocate memory", self.indent()),
            AstNode::ExitProgram { .. } => format!("{}exit program", self.indent()),
            AstNode::SendSignal { .. } => format!("{}send signal", self.indent()),
            
            // Other operations
            AstNode::PropertyAccess { object, property } => {
                format!("{}.{}", self.explain_node(object), property)
            }
            
            // Additional operations
            AstNode::Thread { .. } => format!("{}thread", self.indent()),
            AstNode::Lock { .. } => format!("{}lock", self.indent()),
            AstNode::Emit { .. } => format!("{}emit event", self.indent()),
            AstNode::Watch { .. } => format!("{}watch event", self.indent()),
            AstNode::Auth { .. } => format!("{}authenticate", self.indent()),
            AstNode::Test { .. } => format!("{}test", self.indent()),
            AstNode::Mock { .. } => format!("{}mock", self.indent()),
            AstNode::Benchmark { .. } => format!("{}benchmark", self.indent()),
            AstNode::Encrypt { .. } => format!("{}encrypt", self.indent()),
            AstNode::Decrypt { .. } => format!("{}decrypt", self.indent()),
            AstNode::Sign { .. } => format!("{}sign", self.indent()),
            AstNode::VerifySignature { .. } => format!("{}verify signature", self.indent()),
            AstNode::Infinity => "infinity".to_string(),
            AstNode::Delta { .. } => format!("{}delta", self.indent()),
            AstNode::HttpPatch { .. } => format!("{}HTTP PATCH", self.indent()),
            AstNode::HttpHead { .. } => format!("{}HTTP HEAD", self.indent()),
            AstNode::HttpOptions { .. } => format!("{}HTTP OPTIONS", self.indent()),
            // Bootstrap operations (v1.4)
            AstNode::Length { .. } => format!("{}length", self.indent()),
            AstNode::Index { .. } => format!("{}index access", self.indent()),
            AstNode::ArrayPush { .. } => format!("{}array push", self.indent()),
            AstNode::Add { .. } => format!("{}add", self.indent()),
            AstNode::Subtract { .. } => format!("{}subtract", self.indent()),
            AstNode::Multiply { .. } => format!("{}multiply", self.indent()),
            AstNode::Divide { .. } => format!("{}divide", self.indent()),
            AstNode::Modulo { .. } => format!("{}modulo", self.indent()),
            AstNode::StringConcat { .. } => format!("{}string concat", self.indent()),
            AstNode::CharAt { .. } => format!("{}char at", self.indent()),
            AstNode::Slice { .. } => format!("{}slice", self.indent()),
            AstNode::ArrayLiteral { .. } => format!("{}array literal", self.indent()),
            AstNode::ObjectLiteral { .. } => format!("{}object literal", self.indent()),
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
