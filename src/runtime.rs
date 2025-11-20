//! Runtime for executing Aether AST

use crate::error::{AetherError, Result};
use crate::parser::{AstNode, LiteralValue};
use std::collections::{HashMap, HashSet};
use chrono::Utc;
use rand::Rng;
use regex::Regex;

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
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
        }
    }
}

/// Runtime environment for executing Aether programs
pub struct Runtime {
    variables: HashMap<String, Value>,
    immutable_vars: HashSet<String>,
    max_loop_iterations: usize,
}

impl Runtime {
    /// Create a new runtime
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
            immutable_vars: HashSet::new(),
            max_loop_iterations: 10000,
        }
    }
    
    /// Set maximum loop iterations (for safety)
    pub fn set_max_loop_iterations(&mut self, max: usize) {
        self.max_loop_iterations = max;
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
                // Check immutability before assignment
                if self.immutable_vars.contains(variable) {
                    return Err(AetherError::RuntimeError(
                        format!("Cannot modify immutable variable: {}", variable)
                    ));
                }
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
            
            // Control Flow & Iteration
            AstNode::Loop { body } => {
                // Execute loop with safety limit
                let mut iterations = 0;
                let mut last_value = Value::Null;
                
                loop {
                    if iterations >= self.max_loop_iterations {
                        return Err(AetherError::RuntimeError(
                            format!("Loop exceeded maximum iterations ({})", self.max_loop_iterations)
                        ));
                    }
                    
                    last_value = self.eval_node(body)?;
                    
                    // Check for break condition (if result is Null or false, break)
                    if !last_value.is_truthy() {
                        break;
                    }
                    
                    iterations += 1;
                }
                
                Ok(last_value)
            }
            
            AstNode::ForEach { variable, collection, body } => {
                let coll = if matches!(collection.as_ref(), AstNode::Empty) {
                    self.variables.get("_pipe").cloned().unwrap_or(Value::Null)
                } else {
                    self.eval_node(collection)?
                };
                
                // Simplified: iterate over array
                if let Value::Array(items) = coll {
                    let mut results = Vec::new();
                    for item in items {
                        self.variables.insert(variable.clone(), item);
                        results.push(self.eval_node(body)?);
                    }
                    Ok(Value::Array(results))
                } else {
                    println!("ForEach: no array to iterate");
                    Ok(Value::Null)
                }
            }
            
            AstNode::Filter { predicate } => {
                let coll = self.variables.get("_pipe").cloned().unwrap_or(Value::Null);
                if let Value::Array(items) = coll {
                    let mut filtered = Vec::new();
                    for item in items {
                        self.variables.insert("_item".to_string(), item.clone());
                        let result = self.eval_node(predicate)?;
                        // Include item if predicate is truthy
                        if result.is_truthy() {
                            filtered.push(item);
                        }
                    }
                    Ok(Value::Array(filtered))
                } else {
                    Ok(Value::Null)
                }
            }
            
            AstNode::Reduce { operation, initial } => {
                let coll = self.variables.get("_pipe").cloned().unwrap_or(Value::Null);
                let mut accumulator = self.eval_node(initial)?;
                
                if let Value::Array(items) = coll {
                    for item in items {
                        self.variables.insert("_acc".to_string(), accumulator);
                        self.variables.insert("_item".to_string(), item);
                        accumulator = self.eval_node(operation)?;
                    }
                }
                Ok(accumulator)
            }
            
            AstNode::TryRescue { try_body, rescue_body } => {
                match self.eval_node(try_body) {
                    Ok(val) => Ok(val),
                    Err(_) => {
                        if let Some(rescue) = rescue_body {
                            self.eval_node(rescue)
                        } else {
                            Ok(Value::Null)
                        }
                    }
                }
            }
            
            AstNode::Retry { max_attempts, body } => {
                let attempts = max_attempts.unwrap_or(3);
                let mut last_error = None;
                
                for i in 0..attempts {
                    match self.eval_node(body) {
                        Ok(val) => return Ok(val),
                        Err(e) => {
                            last_error = Some(e);
                            // Only log on intermediate failures, not the last one
                            if i < attempts - 1 {
                                eprintln!("Retry attempt {} failed, retrying...", i + 1);
                            }
                        }
                    }
                }
                
                Err(last_error.unwrap_or_else(|| AetherError::RuntimeError("Retry failed".to_string())))
            }
            
            // Concurrency & Async
            AstNode::Async { body } => {
                // In a real implementation, this would spawn an async task
                // For now, we execute synchronously but mark it as async context
                self.variables.insert("_async_context".to_string(), Value::Boolean(true));
                let result = self.eval_node(body)?;
                self.variables.remove("_async_context");
                Ok(result)
            }
            
            AstNode::Await { expression } => {
                // Check if we're in async context
                let in_async = self.variables.get("_async_context")
                    .and_then(|v| if let Value::Boolean(b) = v { Some(*b) } else { None })
                    .unwrap_or(false);
                
                if !in_async {
                    eprintln!("Warning: Await used outside async context");
                }
                
                // Execute the expression (in real impl, this would await a future)
                self.eval_node(expression)
            }
            
            AstNode::Thread { body } => {
                // In a real implementation, this would spawn a thread
                // For now, execute in current thread with thread context marker
                self.variables.insert("_thread_context".to_string(), Value::Boolean(true));
                let result = self.eval_node(body)?;
                self.variables.remove("_thread_context");
                Ok(result)
            }
            
            AstNode::Lock { body } => {
                // In a real implementation, this would acquire a mutex
                // For now, mark critical section and execute
                self.variables.insert("_lock_acquired".to_string(), Value::Boolean(true));
                let result = self.eval_node(body)?;
                self.variables.remove("_lock_acquired");
                Ok(result)
            }
            
            AstNode::Emit { event } => {
                let evt = self.eval_node(event)?;
                println!("Emit event: {:?}", evt);
                Ok(Value::Boolean(true))
            }
            
            AstNode::Watch { event, handler } => {
                let evt = self.eval_node(event)?;
                println!("Watch event: {:?}", evt);
                self.eval_node(handler)
            }
            
            // Data Manipulation
            AstNode::Split { target, delimiter } => {
                let tgt = if matches!(target.as_ref(), AstNode::Empty) {
                    self.variables.get("_pipe").cloned().unwrap_or(Value::Null)
                } else {
                    self.eval_node(target)?
                };
                
                if let Value::String(s) = tgt {
                    let delim = if let Some(d) = delimiter {
                        match self.eval_node(d)? {
                            Value::String(ds) => ds,
                            _ => " ".to_string(),
                        }
                    } else {
                        " ".to_string()
                    };
                    
                    let parts: Vec<Value> = s.split(&delim)
                        .map(|p| Value::String(p.to_string()))
                        .collect();
                    Ok(Value::Array(parts))
                } else {
                    Ok(Value::Null)
                }
            }
            
            AstNode::Join { elements, separator } => {
                let elems = if matches!(elements.as_ref(), AstNode::Empty) {
                    self.variables.get("_pipe").cloned().unwrap_or(Value::Null)
                } else {
                    self.eval_node(elements)?
                };
                
                if let Value::Array(items) = elems {
                    let sep = if let Some(s) = separator {
                        match self.eval_node(s)? {
                            Value::String(ss) => ss,
                            _ => "".to_string(),
                        }
                    } else {
                        "".to_string()
                    };
                    
                    let strings: Vec<String> = items.iter()
                        .filter_map(|v| v.as_string().map(|s| s.to_string()))
                        .collect();
                    Ok(Value::String(strings.join(&sep)))
                } else {
                    Ok(Value::Null)
                }
            }
            
            AstNode::RegexMatch { pattern, target } => {
                let pat = self.eval_node(pattern)?;
                let tgt = if matches!(target.as_ref(), AstNode::Empty) {
                    self.variables.get("_pipe").cloned().unwrap_or(Value::Null)
                } else {
                    self.eval_node(target)?
                };
                
                // Extract pattern string
                let pattern_str = match pat {
                    Value::String(s) => s,
                    _ => return Ok(Value::Boolean(false)),
                };
                
                // Extract target string
                let target_str = match tgt {
                    Value::String(s) => s,
                    _ => return Ok(Value::Boolean(false)),
                };
                
                // Perform regex match
                match Regex::new(&pattern_str) {
                    Ok(re) => Ok(Value::Boolean(re.is_match(&target_str))),
                    Err(e) => Err(AetherError::RuntimeError(format!("Invalid regex pattern: {}", e))),
                }
            }
            
            AstNode::Equal { left, right } => {
                let l = if matches!(left.as_ref(), AstNode::Empty) {
                    self.variables.get("_pipe").cloned().unwrap_or(Value::Null)
                } else {
                    self.eval_node(left)?
                };
                let r = self.eval_node(right)?;
                Ok(Value::Boolean(l == r))
            }
            
            AstNode::NotEqual { left, right } => {
                let l = if matches!(left.as_ref(), AstNode::Empty) {
                    self.variables.get("_pipe").cloned().unwrap_or(Value::Null)
                } else {
                    self.eval_node(left)?
                };
                let r = self.eval_node(right)?;
                Ok(Value::Boolean(l != r))
            }
            
            AstNode::Immutable { name, value } => {
                // Check if variable is already immutable
                if self.immutable_vars.contains(name) {
                    return Err(AetherError::RuntimeError(
                        format!("Cannot redefine immutable variable: {}", name)
                    ));
                }
                
                let val = self.eval_node(value)?;
                self.variables.insert(name.clone(), val.clone());
                self.immutable_vars.insert(name.clone());
                Ok(val)
            }
            
            // System & Environment
            AstNode::Import { module } => {
                // Mark module as imported in runtime context
                let import_key = format!("_imported_{}", module);
                self.variables.insert(import_key, Value::Boolean(true));
                Ok(Value::Boolean(true))
            }
            
            AstNode::Auth { token } => {
                let tok = self.eval_node(token)?;
                // Store auth token in runtime context
                self.variables.insert("_auth_token".to_string(), tok);
                Ok(Value::Boolean(true))
            }
            
            AstNode::DateTime => {
                // Return actual current time in ISO 8601 format
                let now = Utc::now();
                Ok(Value::String(now.to_rfc3339()))
            }
            
            AstNode::Random => {
                // Generate actual random number between 0 and 1
                let mut rng = rand::thread_rng();
                let random_value: f64 = rng.gen();
                Ok(Value::Number(random_value))
            }
            
            AstNode::Log { message } => {
                let msg = self.eval_node(message)?;
                println!("LOG: {:?}", msg);
                Ok(Value::Null)
            }
            
            AstNode::HttpGet { url } => {
                let url_val = self.eval_node(url)?;
                println!("HTTP GET: {:?}", url_val);
                Ok(Value::Object(HashMap::new()))
            }
        }
    }

    /// Set a variable in the runtime environment
    pub fn set_variable(&mut self, name: String, value: Value) -> Result<()> {
        // Check if variable is immutable (but allow internal _ prefixed vars)
        if !name.starts_with('_') && self.immutable_vars.contains(&name) {
            return Err(AetherError::RuntimeError(
                format!("Cannot modify immutable variable: {}", name)
            ));
        }
        self.variables.insert(name, value);
        Ok(())
    }

    /// Get a variable from the runtime environment
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    /// Check if a variable is immutable
    pub fn is_immutable(&self, name: &str) -> bool {
        self.immutable_vars.contains(name)
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
        runtime.set_variable("x".to_string(), Value::Number(10.0)).unwrap();

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
    
    #[test]
    fn test_runtime_split() {
        let mut runtime = Runtime::new();
        runtime.set_variable("_pipe".to_string(), Value::String("a,b,c".to_string())).unwrap();
        
        let node = AstNode::Split {
            target: Box::new(AstNode::Empty),
            delimiter: Some(Box::new(AstNode::Literal(LiteralValue::String(",".to_string())))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        match result {
            Value::Array(items) => assert_eq!(items.len(), 3),
            _ => panic!("Expected array"),
        }
    }
    
    #[test]
    fn test_runtime_foreach() {
        let mut runtime = Runtime::new();
        let items = vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ];
        
        runtime.set_variable("_pipe".to_string(), Value::Array(items)).unwrap();
        // Execute ForEach by setting collection to Empty and using _pipe
        let node = AstNode::ForEach {
            variable: "x".to_string(),
            collection: Box::new(AstNode::Empty),
            body: Box::new(AstNode::Variable("x".to_string())),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        match result {
            Value::Array(results) => assert_eq!(results.len(), 3),
            _ => panic!("Expected array"),
        }
    }
    #[test]
    fn test_runtime_retry() {
        let mut runtime = Runtime::new();
        let node = AstNode::Retry {
            max_attempts: Some(3),
            body: Box::new(AstNode::Literal(LiteralValue::Number(42.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }
    
    #[test]
    fn test_runtime_try_rescue() {
        let mut runtime = Runtime::new();
        let node = AstNode::TryRescue {
            try_body: Box::new(AstNode::Literal(LiteralValue::Number(42.0))),
            rescue_body: Some(Box::new(AstNode::Literal(LiteralValue::Number(0.0)))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }
    
    #[test]
    fn test_runtime_async() {
        let mut runtime = Runtime::new();
        let node = AstNode::Async {
            body: Box::new(AstNode::Literal(LiteralValue::String("async_result".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::String("async_result".to_string()));
    }
    
    #[test]
    fn test_runtime_log() {
        let mut runtime = Runtime::new();
        let node = AstNode::Log {
            message: Box::new(AstNode::Literal(LiteralValue::String("test message".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Null);
    }
    
    #[test]
    fn test_runtime_equal() {
        let mut runtime = Runtime::new();
        runtime.set_variable("_pipe".to_string(), Value::Number(42.0)).unwrap();
        
        let node = AstNode::Equal {
            left: Box::new(AstNode::Empty),
            right: Box::new(AstNode::Literal(LiteralValue::Number(42.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }
    
    #[test]
    fn test_runtime_datetime() {
        let mut runtime = Runtime::new();
        let node = AstNode::DateTime;
        
        let result = runtime.eval_node(&node).unwrap();
        match result {
            Value::String(_) => {},
            _ => panic!("Expected string timestamp"),
        }
    }
    
    #[test]
    fn test_runtime_immutable() {
        let mut runtime = Runtime::new();
        
        // Define immutable variable
        let node = AstNode::Immutable {
            name: "PI".to_string(),
            value: Box::new(AstNode::Literal(LiteralValue::Number(3.14159))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(3.14159));
        
        // Try to modify it - should fail
        let set_result = runtime.set_variable("PI".to_string(), Value::Number(3.0));
        assert!(set_result.is_err());
        
        // Internal variables should still work
        runtime.set_variable("_pipe".to_string(), Value::Number(1.0)).unwrap();
    }
    
    #[test]
    fn test_runtime_random() {
        let mut runtime = Runtime::new();
        let node = AstNode::Random;
        
        let result1 = runtime.eval_node(&node).unwrap();
        let result2 = runtime.eval_node(&node).unwrap();
        
        // Both should be numbers
        assert!(matches!(result1, Value::Number(_)));
        assert!(matches!(result2, Value::Number(_)));
        
        // They should (very likely) be different
        if let (Value::Number(n1), Value::Number(n2)) = (result1, result2) {
            // With very high probability they're different, but we can't guarantee it
            // So just check they're in valid range
            assert!(n1 >= 0.0 && n1 <= 1.0);
            assert!(n2 >= 0.0 && n2 <= 1.0);
        }
    }
    
    #[test]
    fn test_runtime_regex_match() {
        let mut runtime = Runtime::new();
        runtime.set_variable("_pipe".to_string(), Value::String("test@example.com".to_string())).unwrap();
        
        let node = AstNode::RegexMatch {
            pattern: Box::new(AstNode::Literal(LiteralValue::String(r"^[\w\.-]+@[\w\.-]+\.\w+$".to_string()))),
            target: Box::new(AstNode::Empty),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(true));
        
        // Test non-matching
        runtime.set_variable("_pipe".to_string(), Value::String("not-an-email".to_string())).unwrap();
        let result2 = runtime.eval_node(&node).unwrap();
        assert_eq!(result2, Value::Boolean(false));
    }
    
    #[test]
    fn test_runtime_loop() {
        let mut runtime = Runtime::new();
        runtime.set_max_loop_iterations(5);
        
        // Loop that breaks after a few iterations
        // We'll use a counter pattern
        runtime.set_variable("counter".to_string(), Value::Number(0.0)).unwrap();
        
        // Simple loop that returns null (falsy) immediately
        let node = AstNode::Loop {
            body: Box::new(AstNode::Literal(LiteralValue::Number(0.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }
    
    #[test]
    fn test_runtime_import() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Import {
            module: "http".to_string(),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(true));
        
        // Check that the import is recorded
        let import_key = "_imported_http";
        assert_eq!(runtime.get_variable(import_key), Some(&Value::Boolean(true)));
    }
}
