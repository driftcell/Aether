//! Runtime for executing Aether AST

use crate::error::{AetherError, Result};
use crate::parser::{AstNode, LiteralValue};
use std::collections::HashMap;

/// Runtime value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}

impl Value {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

/// Runtime environment for executing Aether programs
pub struct Runtime {
    variables: HashMap<String, Value>,
}

impl Runtime {
    /// Create a new runtime
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
        }
    }

    /// Execute an AST and return the result
    pub fn execute(&mut self, nodes: Vec<AstNode>) -> Result<Value> {
        let mut last_value = Value::Null;

        for node in nodes {
            last_value = self.eval_node(&node)?;
        }

        Ok(last_value)
    }

    /// Evaluate a single AST node
    fn eval_node(&mut self, node: &AstNode) -> Result<Value> {
        match node {
            AstNode::Function { name, body } => {
                // For now, just execute the body
                // In a full implementation, this would define a callable function
                println!("Defining function: {}", name);
                self.eval_node(body)
            }

            AstNode::Sequence(operations) => {
                let mut last_value = Value::Null;
                for op in operations {
                    last_value = self.eval_node(op)?;
                }
                Ok(last_value)
            }

            AstNode::Input => {
                // Simulate input - in real implementation, this would read from request context
                Ok(Value::Object(HashMap::new()))
            }

            AstNode::Output(value) => {
                let result = self.eval_node(value)?;
                println!("Output: {:?}", result);
                Ok(result)
            }

            AstNode::Pipe { source, operation } => {
                let source_value = self.eval_node(source)?;
                // Set a temporary variable for piped value
                self.variables.insert("_pipe".to_string(), source_value);
                self.eval_node(operation)
            }

            AstNode::PipeInto { value, variable } => {
                let val = self.eval_node(value)?;
                self.variables.insert(variable.clone(), val.clone());
                Ok(val)
            }

            AstNode::Guard { condition: _, then_branch } => {
                // Check if last piped value is null
                let piped = self.variables.get("_pipe").cloned().unwrap_or(Value::Null);
                if piped.is_null() {
                    self.eval_node(then_branch)
                } else {
                    Ok(piped)
                }
            }

            AstNode::Halt(error_code) => {
                let code = self.eval_node(error_code)?;
                Err(AetherError::RuntimeError(format!("Halted with code: {:?}", code)))
            }

            AstNode::Persist(value) => {
                let val = if matches!(value.as_ref(), AstNode::Empty) {
                    self.variables.get("_pipe").cloned().unwrap_or(Value::Null)
                } else {
                    self.eval_node(value)?
                };
                println!("Persisting: {:?}", val);
                Ok(Value::Boolean(true))
            }

            AstNode::JsonParse(source) => {
                let _val = self.eval_node(source)?;
                // Simulate JSON parsing
                Ok(Value::Object(HashMap::new()))
            }

            AstNode::Variable(name) => {
                self.variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| AetherError::RuntimeError(format!("Undefined variable: {}", name)))
            }

            AstNode::Literal(lit) => match lit {
                LiteralValue::String(s) => Ok(Value::String(s.clone())),
                LiteralValue::Number(n) => Ok(Value::Number(*n)),
            },

            AstNode::Empty => Ok(Value::Null),
        }
    }

    /// Set a variable in the runtime environment
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    /// Get a variable from the runtime environment
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_literal() {
        let mut runtime = Runtime::new();
        let node = AstNode::Literal(LiteralValue::Number(42.0));
        let result = runtime.eval_node(&node).unwrap();

        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_runtime_variable() {
        let mut runtime = Runtime::new();
        runtime.set_variable("x".to_string(), Value::Number(10.0));

        let node = AstNode::Variable("x".to_string());
        let result = runtime.eval_node(&node).unwrap();

        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_runtime_sequence() {
        let mut runtime = Runtime::new();
        let nodes = vec![
            AstNode::Literal(LiteralValue::Number(1.0)),
            AstNode::Literal(LiteralValue::Number(2.0)),
        ];
        let node = AstNode::Sequence(nodes);
        let result = runtime.eval_node(&node).unwrap();

        // Should return last value
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_runtime_pipe_into() {
        let mut runtime = Runtime::new();
        let node = AstNode::PipeInto {
            value: Box::new(AstNode::Literal(LiteralValue::String("test".to_string()))),
            variable: "x".to_string(),
        };

        runtime.eval_node(&node).unwrap();
        let var = runtime.get_variable("x").unwrap();
        assert_eq!(var, &Value::String("test".to_string()));
    }
}
