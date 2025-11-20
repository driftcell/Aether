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
    // v1.2 Testing & Debugging
    test_context: Option<TestContext>,
    mocked_targets: HashSet<String>,
    debug_enabled: bool,
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
        Runtime {
            variables: HashMap::new(),
            immutable_vars: HashSet::new(),
            max_loop_iterations: 10000,
            test_context: None,
            mocked_targets: HashSet::new(),
            debug_enabled: false,
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
}
