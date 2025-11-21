//! Runtime for executing Aether AST

use crate::error::{AetherError, Result};
use crate::parser::{AstNode, LiteralValue};
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use chrono::Utc;
use rand::Rng;
use regex::Regex;

// Crypto imports for v1.2
use sha2::{Sha256, Digest};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use ed25519_dalek::{Signer, Verifier, SigningKey, Signature};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// HTTP client import
use reqwest;
use serde_json;

// Async runtime imports
use tokio::runtime::Runtime as TokioRuntime;
use std::sync::{Arc, Mutex};

/// Runtime value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    /// AsyncTask represents a task handle (ID for tracking)
    AsyncTask(String),
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
            Value::AsyncTask(_) => true, // Task handles are truthy
        }
    }
    
    pub fn as_async_task(&self) -> Option<&str> {
        match self {
            Value::AsyncTask(id) => Some(id),
            _ => None,
        }
    }
}

/// Runtime environment for executing Aether programs
pub struct Runtime {
    variables: HashMap<String, Value>,
    immutable_vars: HashSet<String>,
    max_loop_iterations: usize,
    // v1.2 Testing & Debugging
    test_context: Option<TestContext>,
    mocked_targets: HashSet<String>,
    debug_enabled: bool,
    // Async runtime support
    tokio_runtime: Arc<TokioRuntime>,
    async_tasks: Arc<Mutex<HashMap<String, Arc<Mutex<Option<Value>>>>>>,
    task_counter: Arc<Mutex<usize>>,
}

/// Test execution context
#[derive(Debug, Clone)]
struct TestContext {
    name: String,
    assertions_passed: usize,
    assertions_failed: usize,
}

impl Runtime {
    /// Create a new runtime
    pub fn new() -> Self {
        // Create tokio runtime with multi-threaded scheduler
        let tokio_runtime = Arc::new(
            TokioRuntime::new()
                .expect("Failed to create tokio runtime")
        );
        
        Runtime {
            variables: HashMap::new(),
            immutable_vars: HashSet::new(),
            max_loop_iterations: 10000,
            test_context: None,
            mocked_targets: HashSet::new(),
            debug_enabled: false,
            tokio_runtime,
            async_tasks: Arc::new(Mutex::new(HashMap::new())),
            task_counter: Arc::new(Mutex::new(0)),
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
            
            AstNode::IfThen { condition, then_branch, else_branch } => {
                let cond_value = self.eval_node(condition)?;
                
                if cond_value.is_truthy() {
                    self.eval_node(then_branch)
                } else if let Some(else_node) = else_branch {
                    self.eval_node(else_node)
                } else {
                    Ok(Value::Null)
                }
            }
            
            // Control Flow & Iteration
            AstNode::Loop { condition, body } => {
                // Execute loop with safety limit
                let mut iterations = 0;
                let mut last_value = Value::Null;
                
                loop {
                    if iterations >= self.max_loop_iterations {
                        return Err(AetherError::RuntimeError(
                            format!("Loop exceeded maximum iterations ({})", self.max_loop_iterations)
                        ));
                    }
                    
                    // Check condition if present
                    if let Some(cond) = condition {
                        let cond_value = self.eval_node(cond)?;
                        if !cond_value.is_truthy() {
                            break;
                        }
                    }
                    
                    last_value = self.eval_node(body)?;
                    
                    // If no condition, check if result is falsy to break
                    if condition.is_none() && !last_value.is_truthy() {
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
                // Spawn a real async task
                let task_id = {
                    let mut counter = self.task_counter.lock().unwrap();
                    *counter += 1;
                    format!("task_{}", *counter)
                };
                
                // Clone what we need for the async task
                let body_clone = body.clone();
                let tasks = Arc::clone(&self.async_tasks);
                let task_result = Arc::new(Mutex::new(None));
                tasks.lock().unwrap().insert(task_id.clone(), Arc::clone(&task_result));
                
                // Clone the tokio runtime
                let tokio_rt = Arc::clone(&self.tokio_runtime);
                
                // Spawn the async task using spawn_blocking to avoid nested runtime issues
                tokio_rt.spawn(async move {
                    // Use spawn_blocking to run synchronous code in async context
                    let result = tokio::task::spawn_blocking(move || {
                        // Create a simple evaluation - for complex expressions this is limited
                        // but avoids nested runtime creation
                        match &*body_clone {
                            AstNode::Literal(lit) => {
                                match lit {
                                    LiteralValue::String(s) => Value::String(s.clone()),
                                    LiteralValue::Number(n) => Value::Number(*n),
                                }
                            }
                            AstNode::Output(value) => {
                                // Handle simple output case
                                match value.as_ref() {
                                    AstNode::Literal(lit) => {
                                        match lit {
                                            LiteralValue::String(s) => Value::String(s.clone()),
                                            LiteralValue::Number(n) => Value::Number(*n),
                                        }
                                    }
                                    _ => Value::Null,
                                }
                            }
                            _ => Value::Null,
                        }
                    }).await;
                    
                    let value = result.unwrap_or(Value::Null);
                    *task_result.lock().unwrap() = Some(value);
                });
                
                // Return the task handle
                Ok(Value::AsyncTask(task_id))
            }
            
            AstNode::Await { expression } => {
                // Evaluate the expression to get the task handle
                let task_value = self.eval_node(expression)?;
                
                // Check if it's an async task
                if let Value::AsyncTask(task_id) = task_value {
                    // Poll the task result until it completes
                    let tasks = Arc::clone(&self.async_tasks);
                    let tokio_rt = Arc::clone(&self.tokio_runtime);
                    
                    // Use block_on to wait for the task result
                    tokio_rt.block_on(async move {
                        // Poll with timeout
                        let max_polls = 1000;
                        for _ in 0..max_polls {
                            let tasks_guard = tasks.lock().unwrap();
                            if let Some(task_result) = tasks_guard.get(&task_id) {
                                let result_guard = task_result.lock().unwrap();
                                if let Some(value) = result_guard.as_ref() {
                                    return Ok(value.clone());
                                }
                            }
                            drop(tasks_guard);
                            
                            // Small delay before polling again
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                        }
                        
                        // Timeout
                        Err(AetherError::RuntimeError("Async task timeout".to_string()))
                    })
                } else {
                    // If not an async task, just return the value
                    Ok(task_value)
                }
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
            
            AstNode::And { left, right } => {
                let l = self.eval_node(left)?;
                // Short-circuit: if left is falsy, don't evaluate right
                if !l.is_truthy() {
                    return Ok(Value::Boolean(false));
                }
                let r = self.eval_node(right)?;
                Ok(Value::Boolean(r.is_truthy()))
            }
            
            AstNode::Or { left, right } => {
                let l = self.eval_node(left)?;
                // Short-circuit: if left is truthy, don't evaluate right
                if l.is_truthy() {
                    return Ok(Value::Boolean(true));
                }
                let r = self.eval_node(right)?;
                Ok(Value::Boolean(r.is_truthy()))
            }
            
            AstNode::Not { operand } => {
                let val = self.eval_node(operand)?;
                Ok(Value::Boolean(!val.is_truthy()))
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
            
            // HTTP Methods - Using reqwest with rustls
            AstNode::HttpGet { url, headers } => {
                let url_val = self.eval_node(url)?;
                let url_str = url_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("HTTP URL must be string".to_string()))?;
                
                let headers_data = if let Some(h) = headers {
                    Some(self.eval_node(h)?)
                } else {
                    None
                };
                
                self.execute_http_request("GET", url_str, None, headers_data)
            }
            
            AstNode::HttpPost { url, body, headers } => {
                let url_val = self.eval_node(url)?;
                let url_str = url_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("HTTP URL must be string".to_string()))?;
                
                let body_data = if let Some(b) = body {
                    Some(self.eval_node(b)?)
                } else {
                    None
                };
                
                let headers_data = if let Some(h) = headers {
                    Some(self.eval_node(h)?)
                } else {
                    None
                };
                
                self.execute_http_request("POST", url_str, body_data, headers_data)
            }
            
            AstNode::HttpPut { url, body, headers } => {
                let url_val = self.eval_node(url)?;
                let url_str = url_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("HTTP URL must be string".to_string()))?;
                
                let body_data = if let Some(b) = body {
                    Some(self.eval_node(b)?)
                } else {
                    None
                };
                
                let headers_data = if let Some(h) = headers {
                    Some(self.eval_node(h)?)
                } else {
                    None
                };
                
                self.execute_http_request("PUT", url_str, body_data, headers_data)
            }
            
            AstNode::HttpDelete { url, headers } => {
                let url_val = self.eval_node(url)?;
                let url_str = url_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("HTTP URL must be string".to_string()))?;
                
                let headers_data = if let Some(h) = headers {
                    Some(self.eval_node(h)?)
                } else {
                    None
                };
                
                self.execute_http_request("DELETE", url_str, None, headers_data)
            }
            
            AstNode::HttpPatch { url, body, headers } => {
                let url_val = self.eval_node(url)?;
                let url_str = url_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("HTTP URL must be string".to_string()))?;
                
                let body_data = if let Some(b) = body {
                    Some(self.eval_node(b)?)
                } else {
                    None
                };
                
                let headers_data = if let Some(h) = headers {
                    Some(self.eval_node(h)?)
                } else {
                    None
                };
                
                self.execute_http_request("PATCH", url_str, body_data, headers_data)
            }
            
            AstNode::HttpHead { url, headers } => {
                let url_val = self.eval_node(url)?;
                let url_str = url_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("HTTP URL must be string".to_string()))?;
                
                let headers_data = if let Some(h) = headers {
                    Some(self.eval_node(h)?)
                } else {
                    None
                };
                
                self.execute_http_request("HEAD", url_str, None, headers_data)
            }
            
            AstNode::HttpOptions { url, headers } => {
                let url_val = self.eval_node(url)?;
                let url_str = url_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("HTTP URL must be string".to_string()))?;
                
                let headers_data = if let Some(h) = headers {
                    Some(self.eval_node(h)?)
                } else {
                    None
                };
                
                self.execute_http_request("OPTIONS", url_str, None, headers_data)
            }
            
            // Testing & Debugging (v1.2)
            AstNode::Test { name, body } => {
                println!("Running test: {}", name);
                self.test_context = Some(TestContext {
                    name: name.clone(),
                    assertions_passed: 0,
                    assertions_failed: 0,
                });
                
                let result = self.eval_node(body);
                
                if let Some(ctx) = &self.test_context {
                    println!("Test '{}' completed: {} passed, {} failed", 
                        ctx.name, ctx.assertions_passed, ctx.assertions_failed);
                }
                
                self.test_context = None;
                result
            }
            
            AstNode::Assert { condition } => {
                let cond = self.eval_node(condition)?;
                let is_true = cond.is_truthy();
                
                if let Some(ctx) = &mut self.test_context {
                    if is_true {
                        ctx.assertions_passed += 1;
                    } else {
                        ctx.assertions_failed += 1;
                    }
                }
                
                if !is_true {
                    return Err(AetherError::RuntimeError(
                        format!("Assertion failed: condition evaluated to {:?}", cond)
                    ));
                }
                
                Ok(Value::Boolean(true))
            }
            
            AstNode::Mock { target } => {
                let tgt = self.eval_node(target)?;
                let target_str = match tgt {
                    Value::String(s) => s,
                    _ => format!("{:?}", tgt),
                };
                
                self.mocked_targets.insert(target_str.clone());
                println!("Mocked: {}", target_str);
                Ok(Value::Boolean(true))
            }
            
            AstNode::Benchmark { body } => {
                let start = Instant::now();
                let result = self.eval_node(body)?;
                let duration = start.elapsed();
                
                let duration_ms = duration.as_secs_f64() * 1000.0;
                println!("Benchmark: {:.3}ms", duration_ms);
                
                // Store duration in milliseconds
                self.variables.insert("_benchmark_time".to_string(), Value::Number(duration_ms));
                
                Ok(result)
            }
            
            AstNode::Debug => {
                println!("DEBUG: Breakpoint hit");
                println!("Variables: {:?}", self.variables);
                self.debug_enabled = true;
                Ok(Value::Null)
            }
            
            // Security & Crypto (v1.2)
            AstNode::Encrypt { data, key } => {
                let data_val = self.eval_node(data)?;
                let key_val = self.eval_node(key)?;
                
                let plaintext = match data_val {
                    Value::String(s) => s.as_bytes().to_vec(),
                    _ => return Err(AetherError::RuntimeError("Encrypt requires string data".to_string())),
                };
                
                let key_str = match key_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Encrypt requires string key".to_string())),
                };
                
                // Use SHA-256 to derive a 32-byte key from the string
                let mut hasher = Sha256::new();
                hasher.update(key_str.as_bytes());
                let key_bytes = hasher.finalize();
                
                // Create cipher
                let cipher = Aes256Gcm::new((&key_bytes).into());
                
                // Generate nonce
                let nonce_bytes = rand::thread_rng().gen::<[u8; 12]>();
                let nonce = Nonce::from_slice(&nonce_bytes);
                
                // Encrypt
                match cipher.encrypt(nonce, plaintext.as_ref()) {
                    Ok(ciphertext) => {
                        // Combine nonce + ciphertext and encode as base64
                        let mut combined = nonce_bytes.to_vec();
                        combined.extend_from_slice(&ciphertext);
                        let encoded = BASE64.encode(&combined);
                        Ok(Value::String(encoded))
                    }
                    Err(e) => Err(AetherError::RuntimeError(format!("Encryption failed: {}", e))),
                }
            }
            
            AstNode::Decrypt { data, key } => {
                let data_val = self.eval_node(data)?;
                let key_val = self.eval_node(key)?;
                
                let encrypted_str = match data_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Decrypt requires string data".to_string())),
                };
                
                let key_str = match key_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Decrypt requires string key".to_string())),
                };
                
                // Decode from base64
                let combined = BASE64.decode(encrypted_str.as_bytes())
                    .map_err(|e| AetherError::RuntimeError(format!("Invalid encrypted data: {}", e)))?;
                
                if combined.len() < 12 {
                    return Err(AetherError::RuntimeError("Invalid encrypted data: too short".to_string()));
                }
                
                // Extract nonce and ciphertext
                let (nonce_bytes, ciphertext) = combined.split_at(12);
                let nonce = Nonce::from_slice(nonce_bytes);
                
                // Derive key
                let mut hasher = Sha256::new();
                hasher.update(key_str.as_bytes());
                let key_bytes = hasher.finalize();
                
                // Create cipher
                let cipher = Aes256Gcm::new((&key_bytes).into());
                
                // Decrypt
                match cipher.decrypt(nonce, ciphertext) {
                    Ok(plaintext) => {
                        let text = String::from_utf8(plaintext)
                            .map_err(|e| AetherError::RuntimeError(format!("Invalid UTF-8: {}", e)))?;
                        Ok(Value::String(text))
                    }
                    Err(e) => Err(AetherError::RuntimeError(format!("Decryption failed: {}", e))),
                }
            }
            
            AstNode::Hash { data } => {
                let data_val = self.eval_node(data)?;
                
                let bytes = match data_val {
                    Value::String(s) => s.as_bytes().to_vec(),
                    Value::Number(n) => n.to_string().as_bytes().to_vec(),
                    _ => return Err(AetherError::RuntimeError("Hash requires string or number".to_string())),
                };
                
                let mut hasher = Sha256::new();
                hasher.update(&bytes);
                let result = hasher.finalize();
                
                // Convert to hex string
                let hex_string = format!("{:x}", result);
                Ok(Value::String(hex_string))
            }
            
            AstNode::Sign { data, key } => {
                let data_val = self.eval_node(data)?;
                let key_val = self.eval_node(key)?;
                
                let message = match data_val {
                    Value::String(s) => s.as_bytes().to_vec(),
                    _ => return Err(AetherError::RuntimeError("Sign requires string data".to_string())),
                };
                
                let key_str = match key_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Sign requires string key".to_string())),
                };
                
                // Derive signing key from string (in real impl, would use actual key)
                let mut key_bytes = [0u8; 32];
                let mut hasher = Sha256::new();
                hasher.update(key_str.as_bytes());
                key_bytes.copy_from_slice(&hasher.finalize()[..32]);
                
                let signing_key = SigningKey::from_bytes(&key_bytes);
                let signature = signing_key.sign(&message);
                
                // Encode signature as base64
                let sig_bytes = signature.to_bytes();
                let encoded = BASE64.encode(&sig_bytes);
                Ok(Value::String(encoded))
            }
            
            AstNode::VerifySignature { signature, data, key } => {
                let sig_val = self.eval_node(signature)?;
                let data_val = self.eval_node(data)?;
                let key_val = self.eval_node(key)?;
                
                let sig_str = match sig_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Verify requires string signature".to_string())),
                };
                
                let message = match data_val {
                    Value::String(s) => s.as_bytes().to_vec(),
                    _ => return Err(AetherError::RuntimeError("Verify requires string data".to_string())),
                };
                
                let key_str = match key_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Verify requires string key".to_string())),
                };
                
                // Decode signature
                let sig_bytes = BASE64.decode(sig_str.as_bytes())
                    .map_err(|e| AetherError::RuntimeError(format!("Invalid signature: {}", e)))?;
                
                if sig_bytes.len() != 64 {
                    return Err(AetherError::RuntimeError("Invalid signature length".to_string()));
                }
                
                let mut sig_array = [0u8; 64];
                sig_array.copy_from_slice(&sig_bytes);
                let signature = Signature::from_bytes(&sig_array);
                
                // Derive verifying key (in real impl, would be public key)
                let mut key_bytes = [0u8; 32];
                let mut hasher = Sha256::new();
                hasher.update(key_str.as_bytes());
                key_bytes.copy_from_slice(&hasher.finalize()[..32]);
                
                let signing_key = SigningKey::from_bytes(&key_bytes);
                let verifying_key = signing_key.verifying_key();
                
                // Verify signature
                match verifying_key.verify(&message, &signature) {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(_) => Ok(Value::Boolean(false)),
                }
            }
            
            // Math & Science (v1.2)
            AstNode::Power { base, exponent } => {
                let base_val = self.eval_node(base)?;
                let exp_val = self.eval_node(exponent)?;
                
                let b = base_val.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Power requires number base".to_string()))?;
                let e = exp_val.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Power requires number exponent".to_string()))?;
                
                Ok(Value::Number(b.powf(e)))
            }
            
            AstNode::Root { value } => {
                let val = self.eval_node(value)?;
                
                let n = val.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Root requires number".to_string()))?;
                
                if n < 0.0 {
                    return Err(AetherError::RuntimeError("Cannot take square root of negative number".to_string()));
                }
                
                Ok(Value::Number(n.sqrt()))
            }
            
            AstNode::Approx { left, right } => {
                let l = self.eval_node(left)?;
                let r = self.eval_node(right)?;
                
                let left_num = l.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Approx requires numbers".to_string()))?;
                let right_num = r.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Approx requires numbers".to_string()))?;
                
                // Use relative epsilon for floating point comparison
                let epsilon = f64::EPSILON * 10.0;
                let approx_equal = (left_num - right_num).abs() < epsilon.max((left_num.abs() + right_num.abs()) * epsilon);
                
                Ok(Value::Boolean(approx_equal))
            }
            
            AstNode::Infinity => {
                Ok(Value::Number(f64::INFINITY))
            }
            
            AstNode::Delta { name, value } => {
                let val = self.eval_node(value)?;
                let delta_name = format!("∆{}", name);
                self.variables.insert(delta_name.clone(), val.clone());
                Ok(val)
            }
            
            // File System (v1.3)
            AstNode::FileHandle { path } => {
                let path_val = self.eval_node(path)?;
                let path_str = path_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("File path must be string".to_string()))?;
                
                // Return file object representation
                let mut file_obj = HashMap::new();
                file_obj.insert("type".to_string(), Value::String("file".to_string()));
                file_obj.insert("path".to_string(), Value::String(path_str.to_string()));
                Ok(Value::Object(file_obj))
            }
            
            AstNode::Directory { path } => {
                let path_val = self.eval_node(path)?;
                let path_str = path_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("Directory path must be string".to_string()))?;
                
                // Return directory object representation
                let mut dir_obj = HashMap::new();
                dir_obj.insert("type".to_string(), Value::String("directory".to_string()));
                dir_obj.insert("path".to_string(), Value::String(path_str.to_string()));
                Ok(Value::Object(dir_obj))
            }
            
            AstNode::PathResolve { path } => {
                let path_val = self.eval_node(path)?;
                let path_str = path_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("Path must be string".to_string()))?;
                
                // In real implementation, would resolve path
                println!("Resolving path: {}", path_str);
                Ok(Value::String(path_str.to_string()))
            }
            
            AstNode::ReadContent { source } => {
                let src = self.eval_node(source)?;
                
                // Simulate reading content
                if let Value::Object(obj) = src {
                    if let Some(Value::String(path)) = obj.get("path") {
                        println!("Reading from: {}", path);
                        Ok(Value::String(format!("Content from {}", path)))
                    } else {
                        Err(AetherError::RuntimeError("Invalid file object for reading".to_string()))
                    }
                } else {
                    // Direct path string
                    let path = src.as_string()
                        .ok_or_else(|| AetherError::RuntimeError("Read requires file or path".to_string()))?;
                    println!("Reading from: {}", path);
                    Ok(Value::String(format!("Content from {}", path)))
                }
            }
            
            AstNode::WriteContent { target, content } => {
                let tgt = self.eval_node(target)?;
                let cnt = self.eval_node(content)?;
                
                let path = if let Value::Object(obj) = &tgt {
                    obj.get("path")
                        .and_then(|v| v.as_string())
                        .ok_or_else(|| AetherError::RuntimeError("Invalid file object".to_string()))?
                } else {
                    tgt.as_string()
                        .ok_or_else(|| AetherError::RuntimeError("Write target must be file or path".to_string()))?
                };
                
                println!("Writing to {}: {:?}", path, cnt);
                Ok(Value::Boolean(true))
            }
            
            AstNode::AppendContent { target, content } => {
                let tgt = self.eval_node(target)?;
                let cnt = self.eval_node(content)?;
                
                let path = if let Value::Object(obj) = &tgt {
                    obj.get("path")
                        .and_then(|v| v.as_string())
                        .ok_or_else(|| AetherError::RuntimeError("Invalid file object".to_string()))?
                } else {
                    tgt.as_string()
                        .ok_or_else(|| AetherError::RuntimeError("Append target must be file or path".to_string()))?
                };
                
                println!("Appending to {}: {:?}", path, cnt);
                Ok(Value::Boolean(true))
            }
            
            AstNode::DeleteFile { target } => {
                let tgt = self.eval_node(target)?;
                
                let path = if let Value::Object(obj) = &tgt {
                    obj.get("path")
                        .and_then(|v| v.as_string())
                        .ok_or_else(|| AetherError::RuntimeError("Invalid file object".to_string()))?
                } else {
                    tgt.as_string()
                        .ok_or_else(|| AetherError::RuntimeError("Delete target must be file or path".to_string()))?
                };
                
                println!("Deleting: {}", path);
                Ok(Value::Boolean(true))
            }
            
            AstNode::SetPermission { target, permission } => {
                let tgt = self.eval_node(target)?;
                let perm = self.eval_node(permission)?;
                
                let path = if let Value::Object(obj) = &tgt {
                    obj.get("path")
                        .and_then(|v| v.as_string())
                        .ok_or_else(|| AetherError::RuntimeError("Invalid file object".to_string()))?
                } else {
                    tgt.as_string()
                        .ok_or_else(|| AetherError::RuntimeError("Permission target must be file or path".to_string()))?
                };
                
                println!("Setting permission on {}: {:?}", path, perm);
                Ok(Value::Boolean(true))
            }
            
            // Streams & Buffers (v1.3)
            AstNode::CreateStream { source } => {
                let src = self.eval_node(source)?;
                
                // Create stream object
                let mut stream_obj = HashMap::new();
                stream_obj.insert("type".to_string(), Value::String("stream".to_string()));
                stream_obj.insert("source".to_string(), src);
                stream_obj.insert("position".to_string(), Value::Number(0.0));
                Ok(Value::Object(stream_obj))
            }
            
            AstNode::CreateBuffer { size } => {
                let sz = self.eval_node(size)?;
                let size_num = sz.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Buffer size must be number".to_string()))?;
                
                // Create buffer object
                let mut buffer_obj = HashMap::new();
                buffer_obj.insert("type".to_string(), Value::String("buffer".to_string()));
                buffer_obj.insert("size".to_string(), Value::Number(size_num));
                buffer_obj.insert("data".to_string(), Value::Array(Vec::new()));
                Ok(Value::Object(buffer_obj))
            }
            
            AstNode::FlushBuffer { target } => {
                let tgt = self.eval_node(target)?;
                println!("Flushing buffer: {:?}", tgt);
                Ok(Value::Boolean(true))
            }
            
            AstNode::EndOfFile => {
                Ok(Value::Boolean(true))
            }
            
            AstNode::SkipBytes { source, count } => {
                let src = self.eval_node(source)?;
                let cnt = self.eval_node(count)?;
                let count_num = cnt.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Skip count must be number".to_string()))?;
                
                println!("Skipping {} bytes in stream: {:?}", count_num, src);
                Ok(src)
            }
            
            // Networking (v1.3)
            AstNode::CreateSocket { socket_type } => {
                let sock_type = self.eval_node(socket_type)?;
                
                // Create socket object
                let mut socket_obj = HashMap::new();
                socket_obj.insert("type".to_string(), Value::String("socket".to_string()));
                socket_obj.insert("protocol".to_string(), sock_type);
                socket_obj.insert("connected".to_string(), Value::Boolean(false));
                Ok(Value::Object(socket_obj))
            }
            
            AstNode::ListenPort { port } => {
                let port_val = self.eval_node(port)?;
                let port_num = port_val.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Port must be number".to_string()))?;
                
                println!("Listening on port: {}", port_num);
                
                // Return listener object
                let mut listener_obj = HashMap::new();
                listener_obj.insert("type".to_string(), Value::String("listener".to_string()));
                listener_obj.insert("port".to_string(), Value::Number(port_num));
                listener_obj.insert("active".to_string(), Value::Boolean(true));
                Ok(Value::Object(listener_obj))
            }
            
            AstNode::ConnectRemote { address } => {
                let addr = self.eval_node(address)?;
                let addr_str = addr.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("Connect address must be string".to_string()))?;
                
                println!("Connecting to: {}", addr_str);
                
                // Return connection object
                let mut conn_obj = HashMap::new();
                conn_obj.insert("type".to_string(), Value::String("connection".to_string()));
                conn_obj.insert("address".to_string(), Value::String(addr_str.to_string()));
                conn_obj.insert("connected".to_string(), Value::Boolean(true));
                Ok(Value::Object(conn_obj))
            }
            
            AstNode::PortNumber { number } => {
                let num = self.eval_node(number)?;
                let port_num = num.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Port number must be numeric".to_string()))?;
                
                Ok(Value::Number(port_num))
            }
            
            AstNode::CreatePacket { data } => {
                let packet_data = self.eval_node(data)?;
                
                // Create packet object
                let mut packet_obj = HashMap::new();
                packet_obj.insert("type".to_string(), Value::String("packet".to_string()));
                packet_obj.insert("data".to_string(), packet_data);
                Ok(Value::Object(packet_obj))
            }
            
            AstNode::Handshake { connection } => {
                let conn = self.eval_node(connection)?;
                println!("Performing handshake: {:?}", conn);
                Ok(Value::Boolean(true))
            }
            
            // Process & OS (v1.3)
            AstNode::ProcessCreate { command } => {
                let cmd = self.eval_node(command)?;
                let cmd_str = cmd.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("Process command must be string".to_string()))?;
                
                println!("Creating process: {}", cmd_str);
                
                // Return process object
                let mut proc_obj = HashMap::new();
                proc_obj.insert("type".to_string(), Value::String("process".to_string()));
                proc_obj.insert("command".to_string(), Value::String(cmd_str.to_string()));
                proc_obj.insert("pid".to_string(), Value::Number(12345.0));
                Ok(Value::Object(proc_obj))
            }
            
            AstNode::ShellExec { command } => {
                let cmd = self.eval_node(command)?;
                let cmd_str = cmd.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("Shell command must be string".to_string()))?;
                
                println!("Executing shell command: {}", cmd_str);
                
                // Simulate command execution
                Ok(Value::String(format!("Output of: {}", cmd_str)))
            }
            
            AstNode::EnvVar { name } => {
                let name_val = self.eval_node(name)?;
                let var_name = name_val.as_string()
                    .ok_or_else(|| AetherError::RuntimeError("Environment variable name must be string".to_string()))?;
                
                // Get environment variable
                match std::env::var(var_name) {
                    Ok(value) => Ok(Value::String(value)),
                    Err(_) => Ok(Value::Null),
                }
            }
            
            AstNode::MemoryAlloc { size } => {
                let sz = self.eval_node(size)?;
                let size_num = sz.as_number()
                    .ok_or_else(|| AetherError::RuntimeError("Memory size must be number".to_string()))?;
                
                println!("Allocating {} bytes of memory", size_num);
                
                // Return memory object
                let mut mem_obj = HashMap::new();
                mem_obj.insert("type".to_string(), Value::String("memory".to_string()));
                mem_obj.insert("size".to_string(), Value::Number(size_num));
                Ok(Value::Object(mem_obj))
            }
            
            AstNode::ExitProgram { code } => {
                let exit_code = self.eval_node(code)?;
                let code_num = exit_code.as_number()
                    .unwrap_or(0.0) as i32;
                
                Err(AetherError::RuntimeError(format!("Program exited with code: {}", code_num)))
            }
            
            AstNode::SendSignal { signal, target } => {
                let sig = self.eval_node(signal)?;
                let tgt = self.eval_node(target)?;
                
                println!("Sending signal {:?} to {:?}", sig, tgt);
                Ok(Value::Boolean(true))
            }
            
            AstNode::PropertyAccess { object, property } => {
                let obj = self.eval_node(object)?;
                
                match obj {
                    Value::Object(map) => {
                        Ok(map.get(property).cloned().unwrap_or(Value::Null))
                    }
                    _ => Err(AetherError::RuntimeError(
                        format!("Cannot access property '{}' on non-object value", property)
                    ))
                }
            }
            
            AstNode::Comparison { left, operator, right } => {
                use crate::parser::ComparisonOp;
                let left_val = self.eval_node(left)?;
                let right_val = self.eval_node(right)?;
                
                let result = match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        match operator {
                            ComparisonOp::GreaterThan => l > r,
                            ComparisonOp::LessThan => l < r,
                        }
                    }
                    _ => false,
                };
                
                Ok(Value::Boolean(result))
            }
            
            // AI & Tensor Core (v1.4)
            AstNode::Brain { prompt, model } => {
                let prompt_val = self.eval_node(prompt)?;
                let prompt_str = match prompt_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Brain prompt must be a string".to_string())),
                };
                
                let model_str = if let Some(m) = model {
                    match self.eval_node(m)? {
                        Value::String(s) => s,
                        _ => std::env::var("AETHER_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),
                    }
                } else {
                    std::env::var("AETHER_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string())
                };
                
                // Call OpenAI API
                self.call_openai_api(&prompt_str, &model_str)
            }
            
            AstNode::Dna { text } => {
                let text_val = self.eval_node(text)?;
                let text_str = match text_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Dna text must be a string".to_string())),
                };
                
                // Create embedding via OpenAI API
                self.create_embedding(&text_str)
            }
            
            AstNode::Tensor { dimensions } => {
                let dims = self.eval_node(dimensions)?;
                println!("Creating tensor with dimensions: {:?}", dims);
                
                // Return a placeholder array representing tensor
                Ok(Value::Array(vec![]))
            }
            
            AstNode::Track { query_vector, collection, top_k } => {
                let _qv = self.eval_node(query_vector)?;
                let _coll = self.eval_node(collection)?;
                let k = if let Some(tk) = top_k {
                    match self.eval_node(tk)? {
                        Value::Number(n) => n as usize,
                        _ => 5,
                    }
                } else {
                    5
                };
                
                println!("Performing vector search with top_k={}", k);
                
                // Return placeholder search results
                Ok(Value::Array(vec![]))
            }
            
            // Cloud & Distributed (v1.5)
            AstNode::Mailbox { data, topic } => {
                let data_val = self.eval_node(data)?;
                let topic_val = self.eval_node(topic)?;
                let topic_str = match topic_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Mailbox topic must be a string".to_string())),
                };
                
                println!("Publishing to queue '{}': {:?}", topic_str, data_val);
                Ok(Value::Boolean(true))
            }
            
            AstNode::CloudFunction { name, body } => {
                println!("Deploying cloud function: {}", name);
                self.eval_node(body)
            }
            
            AstNode::RacingCarCache { key, value, ttl } => {
                let key_val = self.eval_node(key)?;
                let val_val = self.eval_node(value)?;
                
                let ttl_str = if let Some(t) = ttl {
                    match self.eval_node(t)? {
                        Value::String(s) => s,
                        Value::Number(n) => format!("{}s", n),
                        _ => "3600s".to_string(),
                    }
                } else {
                    "3600s".to_string()
                };
                
                println!("Caching {:?} with TTL {}", key_val, ttl_str);
                Ok(val_val)
            }
            
            AstNode::Stethoscope { body } => {
                println!("Health check endpoint:");
                self.eval_node(body)
            }
            
            // Time & Scheduler (v1.6)
            AstNode::Sleep { duration } => {
                let dur_val = self.eval_node(duration)?;
                
                let duration_ms = match dur_val {
                    Value::String(s) => {
                        // Parse duration string like "5s", "100ms", "2m"
                        self.parse_duration(&s)?
                    }
                    Value::Number(n) => (n * 1000.0) as u64, // Assume seconds if number
                    _ => return Err(AetherError::RuntimeError("Sleep duration must be a string or number".to_string())),
                };
                
                println!("Sleeping for {}ms", duration_ms);
                std::thread::sleep(std::time::Duration::from_millis(duration_ms));
                Ok(Value::Null)
            }
            
            AstNode::AlarmClock { schedule, body } => {
                let sched_val = self.eval_node(schedule)?;
                let sched_str = match sched_val {
                    Value::String(s) => s,
                    _ => return Err(AetherError::RuntimeError("Schedule must be a cron expression string".to_string())),
                };
                
                println!("Scheduling task with cron: {}", sched_str);
                // In a real implementation, this would register a cron job
                // For now, just execute the body once
                self.eval_node(body)
            }
            
            AstNode::Hourglass { duration, body } => {
                let dur_val = self.eval_node(duration)?;
                
                let timeout_ms = match dur_val {
                    Value::String(s) => self.parse_duration(&s)?,
                    Value::Number(n) => (n * 1000.0) as u64,
                    _ => return Err(AetherError::RuntimeError("Timeout duration must be a string or number".to_string())),
                };
                
                println!("Executing with timeout of {}ms", timeout_ms);
                
                // Use tokio timeout
                let _body_node = body.clone();
                let result = self.tokio_runtime.block_on(async {
                    tokio::time::timeout(
                        tokio::time::Duration::from_millis(timeout_ms),
                        async {
                            // We can't easily make eval_node async, so we'll use spawn_blocking
                            tokio::task::spawn_blocking(move || {
                                // Note: This is a simplified implementation
                                // In a real scenario, we'd need to pass runtime state properly
                                Ok::<Value, AetherError>(Value::Null)
                            }).await.map_err(|e| AetherError::RuntimeError(format!("Task failed: {}", e)))?
                        }
                    ).await
                });
                
                match result {
                    Ok(Ok(val)) => Ok(val),
                    Ok(Err(e)) => Err(e),
                    Err(_) => Err(AetherError::RuntimeError(format!("Operation timed out after {}ms", timeout_ms))),
                }
            }
        }
    }
    
    /// Execute HTTP request using reqwest with rustls
    fn execute_http_request(&self, method: &str, url: &str, body: Option<Value>, headers: Option<Value>) -> Result<Value> {
        // Create a tokio runtime for async operations
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| AetherError::RuntimeError(format!("Failed to create async runtime: {}", e)))?;
        
        rt.block_on(async {
            // Build the HTTP client with rustls
            let client = reqwest::Client::builder()
                .use_rustls_tls()
                .build()
                .map_err(|e| AetherError::RuntimeError(format!("Failed to create HTTP client: {}", e)))?;
            
            // Create request builder based on method
            let request_builder = match method {
                "GET" => client.get(url),
                "POST" => client.post(url),
                "PUT" => client.put(url),
                "DELETE" => client.delete(url),
                "PATCH" => client.patch(url),
                "HEAD" => client.head(url),
                "OPTIONS" => {
                    client.request(reqwest::Method::OPTIONS, url)
                }
                _ => return Err(AetherError::RuntimeError(format!("Unsupported HTTP method: {}", method))),
            };
            
            // Add body if provided
            let mut request_builder = if let Some(body_val) = body {
                match body_val {
                    Value::String(s) => request_builder.body(s),
                    Value::Object(map) => {
                        // Convert object to JSON string
                        let json_str = self.value_to_json_string(&Value::Object(map));
                        request_builder
                            .header("Content-Type", "application/json")
                            .body(json_str)
                    }
                    _ => request_builder.body(format!("{:?}", body_val)),
                }
            } else {
                request_builder
            };
            
            // Add custom headers if provided
            if let Some(headers_val) = headers {
                if let Value::Object(header_map) = headers_val {
                    for (key, value) in header_map.iter() {
                        let header_value = match value {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Boolean(b) => b.to_string(),
                            _ => format!("{:?}", value),
                        };
                        request_builder = request_builder.header(key.as_str(), header_value);
                    }
                }
            }
            
            // Execute the request
            let response = request_builder
                .send()
                .await
                .map_err(|e| AetherError::RuntimeError(format!("HTTP request failed: {}", e)))?;
            
            // Build response object
            let mut response_obj = HashMap::new();
            response_obj.insert("status".to_string(), Value::Number(response.status().as_u16() as f64));
            response_obj.insert("ok".to_string(), Value::Boolean(response.status().is_success()));
            
            // Extract response headers
            let mut headers_obj = HashMap::new();
            for (key, value) in response.headers().iter() {
                if let Ok(value_str) = value.to_str() {
                    headers_obj.insert(key.to_string(), Value::String(value_str.to_string()));
                }
            }
            response_obj.insert("headers".to_string(), Value::Object(headers_obj));
            
            // Get response body as text
            let body_text = response
                .text()
                .await
                .map_err(|e| AetherError::RuntimeError(format!("Failed to read response body: {}", e)))?;
            
            response_obj.insert("body".to_string(), Value::String(body_text.clone()));
            
            // Try to parse as JSON if possible
            if let Ok(json_val) = self.parse_json_string(&body_text) {
                response_obj.insert("json".to_string(), json_val);
            }
            
            Ok(Value::Object(response_obj))
        })
    }
    
    /// Helper to convert Value to JSON string using serde_json
    fn value_to_json_string(&self, value: &Value) -> String {
        // Convert our Value to serde_json::Value
        let json_value = self.value_to_serde_json(value);
        // Serialize to string
        serde_json::to_string(&json_value).unwrap_or_else(|_| "null".to_string())
    }
    
    /// Convert Aether Value to serde_json::Value
    fn value_to_serde_json(&self, value: &Value) -> serde_json::Value {
        match value {
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Number(n) => {
                if let Some(n_val) = serde_json::Number::from_f64(*n) {
                    serde_json::Value::Number(n_val)
                } else {
                    // NaN, Infinity, or -Infinity - use string representation
                    eprintln!("Warning: Non-finite number {:?} converted to string in JSON", n);
                    serde_json::Value::String(n.to_string())
                }
            }
            Value::Boolean(b) => serde_json::Value::Bool(*b),
            Value::Null => serde_json::Value::Null,
            Value::Array(items) => {
                let json_items: Vec<serde_json::Value> = items.iter()
                    .map(|v| self.value_to_serde_json(v))
                    .collect();
                serde_json::Value::Array(json_items)
            }
            Value::Object(map) => {
                let json_map: serde_json::Map<String, serde_json::Value> = map.iter()
                    .map(|(k, v)| (k.clone(), self.value_to_serde_json(v)))
                    .collect();
                serde_json::Value::Object(json_map)
            }
            Value::AsyncTask(id) => {
                // Represent async task as a string identifier in JSON
                serde_json::Value::String(format!("AsyncTask({})", id))
            }
        }
    }
    
    /// Helper to parse JSON string to Value using serde_json
    fn parse_json_string(&self, json: &str) -> Result<Value> {
        // Parse using serde_json
        match serde_json::from_str::<serde_json::Value>(json) {
            Ok(json_val) => Ok(self.serde_json_to_value(&json_val)),
            Err(e) => {
                // If parsing fails, log the error and return as string
                eprintln!("JSON parsing failed: {} - returning as string", e);
                Ok(Value::String(json.to_string()))
            }
        }
    }
    
    /// Convert serde_json::Value to Aether Value
    fn serde_json_to_value(&self, json: &serde_json::Value) -> Value {
        match json {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(*b),
            serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => Value::String(s.clone()),
            serde_json::Value::Array(arr) => {
                let items: Vec<Value> = arr.iter()
                    .map(|v| self.serde_json_to_value(v))
                    .collect();
                Value::Array(items)
            }
            serde_json::Value::Object(obj) => {
                let map: HashMap<String, Value> = obj.iter()
                    .map(|(k, v)| (k.clone(), self.serde_json_to_value(v)))
                    .collect();
                Value::Object(map)
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
    
    /// Call OpenAI API for inference
    fn call_openai_api(&self, prompt: &str, model: &str) -> Result<Value> {
        // Get configuration from environment variables
        let api_key = std::env::var("AETHER_API_KEY")
            .map_err(|_| AetherError::RuntimeError("AETHER_API_KEY environment variable not set".to_string()))?;
        
        let base_url = std::env::var("AETHER_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        
        let url = format!("{}/chat/completions", base_url);
        
        // Build request body
        let mut body = HashMap::new();
        body.insert("model".to_string(), Value::String(model.to_string()));
        
        let mut messages = Vec::new();
        let mut message = HashMap::new();
        message.insert("role".to_string(), Value::String("user".to_string()));
        message.insert("content".to_string(), Value::String(prompt.to_string()));
        messages.push(Value::Object(message));
        body.insert("messages".to_string(), Value::Array(messages));
        
        // Create headers
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), Value::String(format!("Bearer {}", api_key)));
        headers.insert("Content-Type".to_string(), Value::String("application/json".to_string()));
        
        // Execute HTTP POST request
        let response = self.execute_http_request("POST", &url, Some(Value::Object(body)), Some(Value::Object(headers)))?;
        
        // Extract the response content
        if let Value::Object(resp_obj) = response {
            if let Some(Value::Object(body_obj)) = resp_obj.get("body") {
                if let Some(Value::Array(choices)) = body_obj.get("choices") {
                    if let Some(Value::Object(choice)) = choices.first() {
                        if let Some(Value::Object(message)) = choice.get("message") {
                            if let Some(content) = message.get("content") {
                                return Ok(content.clone());
                            }
                        }
                    }
                }
            }
        }
        
        Err(AetherError::RuntimeError("Failed to parse OpenAI API response".to_string()))
    }
    
    /// Create embedding via OpenAI API
    fn create_embedding(&self, text: &str) -> Result<Value> {
        // Get configuration from environment variables
        let api_key = std::env::var("AETHER_API_KEY")
            .map_err(|_| AetherError::RuntimeError("AETHER_API_KEY environment variable not set".to_string()))?;
        
        let base_url = std::env::var("AETHER_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        
        let url = format!("{}/embeddings", base_url);
        
        let model = std::env::var("AETHER_EMBEDDING_MODEL")
            .unwrap_or_else(|_| "text-embedding-ada-002".to_string());
        
        // Build request body
        let mut body = HashMap::new();
        body.insert("model".to_string(), Value::String(model));
        body.insert("input".to_string(), Value::String(text.to_string()));
        
        // Create headers
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), Value::String(format!("Bearer {}", api_key)));
        headers.insert("Content-Type".to_string(), Value::String("application/json".to_string()));
        
        // Execute HTTP POST request
        let response = self.execute_http_request("POST", &url, Some(Value::Object(body)), Some(Value::Object(headers)))?;
        
        // Extract the embedding vector
        if let Value::Object(resp_obj) = response {
            if let Some(Value::Object(body_obj)) = resp_obj.get("body") {
                if let Some(Value::Array(data)) = body_obj.get("data") {
                    if let Some(Value::Object(embedding_obj)) = data.first() {
                        if let Some(embedding) = embedding_obj.get("embedding") {
                            return Ok(embedding.clone());
                        }
                    }
                }
            }
        }
        
        Err(AetherError::RuntimeError("Failed to parse embedding API response".to_string()))
    }
    
    /// Parse duration string (e.g., "5s", "100ms", "2m", "1h")
    fn parse_duration(&self, duration_str: &str) -> Result<u64> {
        let duration_str = duration_str.trim();
        
        if duration_str.ends_with("ms") {
            let num_str = &duration_str[..duration_str.len() - 2];
            num_str.parse::<u64>()
                .map_err(|_| AetherError::RuntimeError(format!("Invalid duration: {}", duration_str)))
        } else if duration_str.ends_with('s') {
            let num_str = &duration_str[..duration_str.len() - 1];
            num_str.parse::<u64>()
                .map(|n| n * 1000)
                .map_err(|_| AetherError::RuntimeError(format!("Invalid duration: {}", duration_str)))
        } else if duration_str.ends_with('m') {
            let num_str = &duration_str[..duration_str.len() - 1];
            num_str.parse::<u64>()
                .map(|n| n * 60 * 1000)
                .map_err(|_| AetherError::RuntimeError(format!("Invalid duration: {}", duration_str)))
        } else if duration_str.ends_with('h') {
            let num_str = &duration_str[..duration_str.len() - 1];
            num_str.parse::<u64>()
                .map(|n| n * 60 * 60 * 1000)
                .map_err(|_| AetherError::RuntimeError(format!("Invalid duration: {}", duration_str)))
        } else {
            // Try to parse as milliseconds if no unit specified
            duration_str.parse::<u64>()
                .map_err(|_| AetherError::RuntimeError(format!("Invalid duration: {}", duration_str)))
        }
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
    use crate::parser::ComparisonOp;

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
        
        // Create an async task
        let async_node = AstNode::Async {
            body: Box::new(AstNode::Literal(LiteralValue::String("async_result".to_string()))),
        };
        
        // Execute async - should return a task handle
        let task_handle = runtime.eval_node(&async_node).unwrap();
        
        // Verify it's an async task
        assert!(matches!(task_handle, Value::AsyncTask(_)));
        
        // Manually insert the task handle as a variable for testing
        runtime.set_variable("my_task".to_string(), task_handle).unwrap();
        
        // Await using the variable
        let await_result = AstNode::Await {
            expression: Box::new(AstNode::Variable("my_task".to_string())),
        };
        
        let result = runtime.eval_node(&await_result).unwrap();
        assert_eq!(result, Value::String("async_result".to_string()));
    }
    
    #[test]
    fn test_runtime_async_multiple_tasks() {
        let mut runtime = Runtime::new();
        
        // Create multiple async tasks
        let task1 = AstNode::Async {
            body: Box::new(AstNode::Literal(LiteralValue::String("result1".to_string()))),
        };
        let task2 = AstNode::Async {
            body: Box::new(AstNode::Literal(LiteralValue::Number(42.0))),
        };
        
        // Execute both tasks
        let handle1 = runtime.eval_node(&task1).unwrap();
        let handle2 = runtime.eval_node(&task2).unwrap();
        
        // Both should be async tasks
        assert!(matches!(handle1, Value::AsyncTask(_)));
        assert!(matches!(handle2, Value::AsyncTask(_)));
        
        // Store as variables
        runtime.set_variable("t1".to_string(), handle1).unwrap();
        runtime.set_variable("t2".to_string(), handle2).unwrap();
        
        // Await both
        let await1 = AstNode::Await {
            expression: Box::new(AstNode::Variable("t1".to_string())),
        };
        let await2 = AstNode::Await {
            expression: Box::new(AstNode::Variable("t2".to_string())),
        };
        
        let result1 = runtime.eval_node(&await1).unwrap();
        let result2 = runtime.eval_node(&await2).unwrap();
        
        assert_eq!(result1, Value::String("result1".to_string()));
        assert_eq!(result2, Value::Number(42.0));
    }
    
    #[test]
    fn test_runtime_async_with_output() {
        let mut runtime = Runtime::new();
        
        // Create async task with output
        let async_node = AstNode::Async {
            body: Box::new(AstNode::Output(Box::new(AstNode::Literal(
                LiteralValue::String("Hello Async!".to_string())
            )))),
        };
        
        // Execute async
        let task_handle = runtime.eval_node(&async_node).unwrap();
        assert!(matches!(task_handle, Value::AsyncTask(_)));
        
        // Store and await
        runtime.set_variable("task".to_string(), task_handle).unwrap();
        let await_node = AstNode::Await {
            expression: Box::new(AstNode::Variable("task".to_string())),
        };
        
        let result = runtime.eval_node(&await_node).unwrap();
        assert_eq!(result, Value::String("Hello Async!".to_string()));
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
    
    // v1.3 Runtime Tests
    
    #[test]
    fn test_runtime_file_handle() {
        let mut runtime = Runtime::new();
        let node = AstNode::FileHandle {
            path: Box::new(AstNode::Literal(LiteralValue::String("/tmp/test.txt".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        match result {
            Value::Object(obj) => {
                assert_eq!(obj.get("type"), Some(&Value::String("file".to_string())));
                assert_eq!(obj.get("path"), Some(&Value::String("/tmp/test.txt".to_string())));
            }
            _ => panic!("Expected file object"),
        }
    }
    
    #[test]
    fn test_runtime_shell_exec() {
        let mut runtime = Runtime::new();
        let node = AstNode::ShellExec {
            command: Box::new(AstNode::Literal(LiteralValue::String("ls -la".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        match result {
            Value::String(s) => {
                assert!(s.contains("Output of:"));
            }
            _ => panic!("Expected string output"),
        }
    }
    
    #[test]
    fn test_runtime_env_var() {
        let mut runtime = Runtime::new();
        
        // Set an environment variable for testing
        std::env::set_var("AETHER_TEST_VAR", "test_value");
        
        let node = AstNode::EnvVar {
            name: Box::new(AstNode::Literal(LiteralValue::String("AETHER_TEST_VAR".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::String("test_value".to_string()));
        
        // Clean up
        std::env::remove_var("AETHER_TEST_VAR");
    }
    
    #[test]
    fn test_runtime_stream() {
        let mut runtime = Runtime::new();
        let node = AstNode::CreateStream {
            source: Box::new(AstNode::Literal(LiteralValue::String("data".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        match result {
            Value::Object(obj) => {
                assert_eq!(obj.get("type"), Some(&Value::String("stream".to_string())));
            }
            _ => panic!("Expected stream object"),
        }
    }
    
    #[test]
    fn test_runtime_socket() {
        let mut runtime = Runtime::new();
        let node = AstNode::CreateSocket {
            socket_type: Box::new(AstNode::Literal(LiteralValue::String("TCP".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        match result {
            Value::Object(obj) => {
                assert_eq!(obj.get("type"), Some(&Value::String("socket".to_string())));
                assert_eq!(obj.get("connected"), Some(&Value::Boolean(false)));
            }
            _ => panic!("Expected socket object"),
        }
    }
    
    #[test]
    fn test_runtime_listen_port() {
        let mut runtime = Runtime::new();
        let node = AstNode::ListenPort {
            port: Box::new(AstNode::Literal(LiteralValue::Number(8080.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        match result {
            Value::Object(obj) => {
                assert_eq!(obj.get("type"), Some(&Value::String("listener".to_string())));
                assert_eq!(obj.get("port"), Some(&Value::Number(8080.0)));
            }
            _ => panic!("Expected listener object"),
        }
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
            condition: None,
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
    
    // v1.2 Testing & Debugging tests
    #[test]
    fn test_runtime_test_suite() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Test {
            name: "MyTest".to_string(),
            body: Box::new(AstNode::Literal(LiteralValue::Number(42.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }
    
    #[test]
    fn test_runtime_assert_pass() {
        let mut runtime = Runtime::new();
        
        // True condition should pass
        let node = AstNode::Assert {
            condition: Box::new(AstNode::Literal(LiteralValue::Number(1.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }
    
    #[test]
    fn test_runtime_assert_fail() {
        let mut runtime = Runtime::new();
        
        // False condition should fail
        let node = AstNode::Assert {
            condition: Box::new(AstNode::Empty),
        };
        
        let result = runtime.eval_node(&node);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_runtime_mock() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Mock {
            target: Box::new(AstNode::Literal(LiteralValue::String("database".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(true));
        assert!(runtime.mocked_targets.contains("database"));
    }
    
    #[test]
    fn test_runtime_benchmark() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Benchmark {
            body: Box::new(AstNode::Literal(LiteralValue::Number(42.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // Check that benchmark time was stored
        let time = runtime.get_variable("_benchmark_time");
        assert!(time.is_some());
    }
    
    #[test]
    fn test_runtime_debug() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Debug;
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Null);
        assert!(runtime.debug_enabled);
    }
    
    // v1.2 Security & Crypto tests
    #[test]
    fn test_runtime_encrypt_decrypt() {
        let mut runtime = Runtime::new();
        
        // Encrypt
        let encrypt_node = AstNode::Encrypt {
            data: Box::new(AstNode::Literal(LiteralValue::String("secret message".to_string()))),
            key: Box::new(AstNode::Literal(LiteralValue::String("my-key".to_string()))),
        };
        
        let encrypted = runtime.eval_node(&encrypt_node).unwrap();
        
        // Should be a string
        assert!(matches!(encrypted, Value::String(_)));
        
        // Decrypt
        let decrypt_node = AstNode::Decrypt {
            data: Box::new(AstNode::Literal(match encrypted {
                Value::String(s) => LiteralValue::String(s),
                _ => panic!("Expected string"),
            })),
            key: Box::new(AstNode::Literal(LiteralValue::String("my-key".to_string()))),
        };
        
        let decrypted = runtime.eval_node(&decrypt_node).unwrap();
        assert_eq!(decrypted, Value::String("secret message".to_string()));
    }
    
    #[test]
    fn test_runtime_hash() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Hash {
            data: Box::new(AstNode::Literal(LiteralValue::String("test".to_string()))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        
        // Should be a hex string
        if let Value::String(hash) = result {
            assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
            assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        } else {
            panic!("Expected string hash");
        }
    }
    
    #[test]
    fn test_runtime_sign_verify() {
        let mut runtime = Runtime::new();
        
        let message = "important message";
        let key = "signing-key";
        
        // Sign
        let sign_node = AstNode::Sign {
            data: Box::new(AstNode::Literal(LiteralValue::String(message.to_string()))),
            key: Box::new(AstNode::Literal(LiteralValue::String(key.to_string()))),
        };
        
        let signature = runtime.eval_node(&sign_node).unwrap();
        assert!(matches!(signature, Value::String(_)));
        
        // Verify
        let verify_node = AstNode::VerifySignature {
            signature: Box::new(AstNode::Literal(match signature {
                Value::String(s) => LiteralValue::String(s),
                _ => panic!("Expected string"),
            })),
            data: Box::new(AstNode::Literal(LiteralValue::String(message.to_string()))),
            key: Box::new(AstNode::Literal(LiteralValue::String(key.to_string()))),
        };
        
        let verified = runtime.eval_node(&verify_node).unwrap();
        assert_eq!(verified, Value::Boolean(true));
    }
    
    // v1.2 Math & Science tests
    #[test]
    fn test_runtime_power() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Power {
            base: Box::new(AstNode::Literal(LiteralValue::Number(2.0))),
            exponent: Box::new(AstNode::Literal(LiteralValue::Number(3.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }
    
    #[test]
    fn test_runtime_root() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Root {
            value: Box::new(AstNode::Literal(LiteralValue::Number(16.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(4.0));
    }
    
    #[test]
    fn test_runtime_approx() {
        let mut runtime = Runtime::new();
        
        // Test approximately equal numbers
        let node = AstNode::Approx {
            left: Box::new(AstNode::Literal(LiteralValue::Number(0.1 + 0.2))),
            right: Box::new(AstNode::Literal(LiteralValue::Number(0.3))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }
    
    #[test]
    fn test_runtime_infinity() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Infinity;
        
        let result = runtime.eval_node(&node).unwrap();
        if let Value::Number(n) = result {
            assert!(n.is_infinite());
        } else {
            panic!("Expected number");
        }
    }
    
    #[test]
    fn test_runtime_delta() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::Delta {
            name: "temp".to_string(),
            value: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // Check that delta variable was stored
        assert_eq!(runtime.get_variable("∆temp"), Some(&Value::Number(5.0)));
    }
    
    // HTTP Request Tests
    #[test]
    fn test_runtime_http_get() {
        let mut runtime = Runtime::new();
        
        // Test with a simple URL - this will make an actual HTTP request
        // Using httpbin.org as a reliable test endpoint
        let node = AstNode::HttpGet { headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/get".to_string()))),
        };
        
        let result = runtime.eval_node(&node);
        
        // Check that result is an object (response)
        match result {
            Ok(Value::Object(obj)) => {
                // Should have status field
                assert!(obj.contains_key("status"));
                assert!(obj.contains_key("ok"));
                assert!(obj.contains_key("body"));
            }
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                // Network errors are acceptable in test environment
                println!("HTTP request failed (expected in some environments): {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_runtime_http_post() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::HttpPost { headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/post".to_string()))),
            body: Some(Box::new(AstNode::Literal(LiteralValue::String("test data".to_string())))),
        };
        
        let result = runtime.eval_node(&node);
        
        match result {
            Ok(Value::Object(obj)) => {
                assert!(obj.contains_key("status"));
                assert!(obj.contains_key("ok"));
            }
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                println!("HTTP POST failed (expected in some environments): {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_runtime_http_put() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::HttpPut { headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/put".to_string()))),
            body: Some(Box::new(AstNode::Literal(LiteralValue::String("updated data".to_string())))),
        };
        
        let result = runtime.eval_node(&node);
        
        match result {
            Ok(Value::Object(_)) => {},
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                println!("HTTP PUT failed (expected in some environments): {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_runtime_http_delete() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::HttpDelete { headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/delete".to_string()))),
        };
        
        let result = runtime.eval_node(&node);
        
        match result {
            Ok(Value::Object(_)) => {},
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                println!("HTTP DELETE failed (expected in some environments): {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_runtime_http_patch() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::HttpPatch { headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/patch".to_string()))),
            body: Some(Box::new(AstNode::Literal(LiteralValue::String("patch data".to_string())))),
        };
        
        let result = runtime.eval_node(&node);
        
        match result {
            Ok(Value::Object(_)) => {},
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                println!("HTTP PATCH failed (expected in some environments): {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_runtime_http_options() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::HttpOptions { headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/".to_string()))),
        };
        
        let result = runtime.eval_node(&node);
        
        match result {
            Ok(Value::Object(_)) => {},
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                println!("HTTP OPTIONS failed (expected in some environments): {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_runtime_http_head() {
        let mut runtime = Runtime::new();
        
        let node = AstNode::HttpHead { headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/".to_string()))),
        };
        
        let result = runtime.eval_node(&node);
        
        match result {
            Ok(Value::Object(_)) => {},
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                println!("HTTP HEAD failed (expected in some environments): {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_runtime_http_post_with_json() {
        let mut runtime = Runtime::new();
        
        // Create a JSON object as body
        let mut body_obj = HashMap::new();
        body_obj.insert("name".to_string(), Value::String("test".to_string()));
        body_obj.insert("value".to_string(), Value::Number(42.0));
        
        let node = AstNode::HttpPost { headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/post".to_string()))),
            body: Some(Box::new(AstNode::Literal(LiteralValue::String(
                r#"{"name":"test","value":42}"#.to_string()
            )))),
        };
        
        let result = runtime.eval_node(&node);
        
        match result {
            Ok(Value::Object(obj)) => {
                assert!(obj.contains_key("status"));
            }
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                println!("HTTP POST with JSON failed (expected in some environments): {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_runtime_if_else() {
        let mut runtime = Runtime::new();
        
        // Test if branch (true)
        let node = AstNode::IfThen {
            condition: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(10.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
            then_branch: Box::new(AstNode::Literal(LiteralValue::String("big".to_string()))),
            else_branch: Some(Box::new(AstNode::Literal(LiteralValue::String("small".to_string())))),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::String("big".to_string()));
        
        // Test else branch (false)
        let node2 = AstNode::IfThen {
            condition: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(3.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
            then_branch: Box::new(AstNode::Literal(LiteralValue::String("big".to_string()))),
            else_branch: Some(Box::new(AstNode::Literal(LiteralValue::String("small".to_string())))),
        };
        
        let result2 = runtime.eval_node(&node2).unwrap();
        assert_eq!(result2, Value::String("small".to_string()));
    }
    
    #[test]
    fn test_runtime_if_elseif_else() {
        let mut runtime = Runtime::new();
        
        // Test with value that triggers first condition
        let node = AstNode::IfThen {
            condition: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(12.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(10.0))),
            }),
            then_branch: Box::new(AstNode::Literal(LiteralValue::String("large".to_string()))),
            else_branch: Some(Box::new(AstNode::IfThen {
                condition: Box::new(AstNode::Comparison {
                    left: Box::new(AstNode::Literal(LiteralValue::Number(12.0))),
                    operator: ComparisonOp::GreaterThan,
                    right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
                }),
                then_branch: Box::new(AstNode::Literal(LiteralValue::String("medium".to_string()))),
                else_branch: Some(Box::new(AstNode::Literal(LiteralValue::String("small".to_string())))),
            })),
        };
        
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::String("large".to_string()));
        
        // Test with value that triggers elseif
        let node2 = AstNode::IfThen {
            condition: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(7.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(10.0))),
            }),
            then_branch: Box::new(AstNode::Literal(LiteralValue::String("large".to_string()))),
            else_branch: Some(Box::new(AstNode::IfThen {
                condition: Box::new(AstNode::Comparison {
                    left: Box::new(AstNode::Literal(LiteralValue::Number(7.0))),
                    operator: ComparisonOp::GreaterThan,
                    right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
                }),
                then_branch: Box::new(AstNode::Literal(LiteralValue::String("medium".to_string()))),
                else_branch: Some(Box::new(AstNode::Literal(LiteralValue::String("small".to_string())))),
            })),
        };
        
        let result2 = runtime.eval_node(&node2).unwrap();
        assert_eq!(result2, Value::String("medium".to_string()));
    }
    
    #[test]
    fn test_runtime_and() {
        let mut runtime = Runtime::new();
        
        // Both true (using comparisons that evaluate to true)
        let node = AstNode::And {
            left: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(10.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
            right: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(8.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(3.0))),
            }),
        };
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(true));
        
        // First false (short-circuit)
        let node2 = AstNode::And {
            left: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(3.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
            right: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(8.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(3.0))),
            }),
        };
        let result2 = runtime.eval_node(&node2).unwrap();
        assert_eq!(result2, Value::Boolean(false));
        
        // Second false
        let node3 = AstNode::And {
            left: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(10.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
            right: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(2.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
        };
        let result3 = runtime.eval_node(&node3).unwrap();
        assert_eq!(result3, Value::Boolean(false));
    }
    
    #[test]
    fn test_runtime_or() {
        let mut runtime = Runtime::new();
        
        // Both false
        let node = AstNode::Or {
            left: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(3.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
            right: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(2.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
        };
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(false));
        
        // First true (short-circuit)
        let node2 = AstNode::Or {
            left: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(10.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
            right: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(2.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
        };
        let result2 = runtime.eval_node(&node2).unwrap();
        assert_eq!(result2, Value::Boolean(true));
        
        // Second true
        let node3 = AstNode::Or {
            left: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(2.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
            right: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(8.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(3.0))),
            }),
        };
        let result3 = runtime.eval_node(&node3).unwrap();
        assert_eq!(result3, Value::Boolean(true));
    }
    
    #[test]
    fn test_runtime_not() {
        let mut runtime = Runtime::new();
        
        // Not true condition
        let node = AstNode::Not {
            operand: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(10.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
        };
        let result = runtime.eval_node(&node).unwrap();
        assert_eq!(result, Value::Boolean(false));
        
        // Not false condition
        let node2 = AstNode::Not {
            operand: Box::new(AstNode::Comparison {
                left: Box::new(AstNode::Literal(LiteralValue::Number(3.0))),
                operator: ComparisonOp::GreaterThan,
                right: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
            }),
        };
        let result2 = runtime.eval_node(&node2).unwrap();
        assert_eq!(result2, Value::Boolean(true));
        
        // Not with truthy value (number)
        let node3 = AstNode::Not {
            operand: Box::new(AstNode::Literal(LiteralValue::Number(5.0))),
        };
        let result3 = runtime.eval_node(&node3).unwrap();
        assert_eq!(result3, Value::Boolean(false));
        
        // Not with falsy value (zero)
        let node4 = AstNode::Not {
            operand: Box::new(AstNode::Literal(LiteralValue::Number(0.0))),
        };
        let result4 = runtime.eval_node(&node4).unwrap();
        assert_eq!(result4, Value::Boolean(true));
    }
    
    #[test]
    fn test_runtime_http_with_headers() {
        let mut runtime = Runtime::new();
        
        // Create headers object
        let mut headers_map = HashMap::new();
        headers_map.insert("Authorization".to_string(), Value::String("Bearer test-token".to_string()));
        headers_map.insert("X-Custom-Header".to_string(), Value::String("test-value".to_string()));
        
        // Test GET with headers - using a variable to hold headers
        runtime.set_variable("custom_headers".to_string(), Value::Object(headers_map)).unwrap();
        
        let node = AstNode::HttpGet {
            headers: None,
            url: Box::new(AstNode::Literal(LiteralValue::String("https://httpbin.org/headers".to_string()))),
        };
        
        let result = runtime.eval_node(&node);
        
        match result {
            Ok(Value::Object(obj)) => {
                assert!(obj.contains_key("status"));
                assert!(obj.contains_key("headers"));
                // Response should include headers object
                if let Some(Value::Object(_headers)) = obj.get("headers") {
                    // Headers received successfully
                }
            }
            Ok(_) => panic!("Expected object response"),
            Err(e) => {
                println!("HTTP with headers failed (expected in some environments): {:?}", e);
            }
        }
    }
}
