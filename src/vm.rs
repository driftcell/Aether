//! Virtual Machine for executing Aether bytecode

use crate::bytecode::{BytecodeProgram, Opcode};
use crate::error::{AetherError, Result};
use crate::runtime::Value;
use std::collections::{HashMap, HashSet};
use chrono::Utc;
use rand::Rng;
use sha2::{Sha256, Digest};

/// Virtual Machine for bytecode execution
pub struct VM {
    /// Bytecode program
    program: BytecodeProgram,
    /// Program counter
    pc: usize,
    /// Value stack
    stack: Vec<Value>,
    /// Variables
    variables: HashMap<String, Value>,
    /// Immutable variables
    immutable_vars: HashSet<String>,
    /// Call stack for function returns
    call_stack: Vec<usize>,
    /// Maximum iterations for safety
    max_iterations: usize,
}

/// Default maximum iteration limit
const DEFAULT_MAX_ITERATIONS: usize = 10000;

/// Epsilon for approximate equality comparisons
const APPROX_EPSILON: f64 = 0.000001;

impl VM {
    /// Create a new VM with a bytecode program
    pub fn new(program: BytecodeProgram) -> Self {
        VM {
            program,
            pc: 0,
            stack: Vec::new(),
            variables: HashMap::new(),
            immutable_vars: HashSet::new(),
            call_stack: Vec::new(),
            max_iterations: DEFAULT_MAX_ITERATIONS,
        }
    }
    
    /// Execute the bytecode program
    pub fn execute(&mut self) -> Result<Value> {
        let mut iteration_count = 0;
        
        while self.pc < self.program.code.len() {
            iteration_count += 1;
            if iteration_count > self.max_iterations {
                return Err(AetherError::RuntimeError(
                    "Maximum iteration count exceeded".to_string()
                ));
            }
            
            let opcode_byte = self.program.code[self.pc];
            let opcode = Opcode::from_byte(opcode_byte)?;
            self.pc += 1;
            
            match opcode {
                Opcode::PushNull => {
                    self.stack.push(Value::Null);
                }
                
                Opcode::PushBool => {
                    let value = self.read_u8()?;
                    self.stack.push(Value::Boolean(value != 0));
                }
                
                Opcode::PushNumber => {
                    let value = self.read_f64()?;
                    self.stack.push(Value::Number(value));
                }
                
                Opcode::PushString => {
                    let idx = self.read_u32()? as usize;
                    let string = self.program.constants.get(idx)
                        .ok_or_else(|| AetherError::RuntimeError(
                            format!("Invalid constant index: {}", idx)
                        ))?
                        .clone();
                    self.stack.push(Value::String(string));
                }
                
                Opcode::Pop => {
                    self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                }
                
                Opcode::Dup => {
                    let value = self.stack.last()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?
                        .clone();
                    self.stack.push(value);
                }
                
                Opcode::LoadVar => {
                    let idx = self.read_u32()? as usize;
                    let name = self.program.constants.get(idx)
                        .ok_or_else(|| AetherError::RuntimeError(
                            format!("Invalid constant index: {}", idx)
                        ))?;
                    let value = self.variables.get(name)
                        .unwrap_or(&Value::Null)
                        .clone();
                    self.stack.push(value);
                }
                
                Opcode::StoreVar => {
                    let idx = self.read_u32()? as usize;
                    let name = self.program.constants.get(idx)
                        .ok_or_else(|| AetherError::RuntimeError(
                            format!("Invalid constant index: {}", idx)
                        ))?
                        .clone();
                    
                    if self.immutable_vars.contains(&name) {
                        return Err(AetherError::RuntimeError(
                            format!("Cannot modify immutable variable: {}", name)
                        ));
                    }
                    
                    let value = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.variables.insert(name, value);
                }
                
                Opcode::StoreImmutable => {
                    let idx = self.read_u32()? as usize;
                    let name = self.program.constants.get(idx)
                        .ok_or_else(|| AetherError::RuntimeError(
                            format!("Invalid constant index: {}", idx)
                        ))?
                        .clone();
                    
                    let value = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.variables.insert(name.clone(), value);
                    self.immutable_vars.insert(name);
                }
                
                Opcode::Add => {
                    let right = self.pop_number()?;
                    let left = self.pop_number()?;
                    self.stack.push(Value::Number(left + right));
                }
                
                Opcode::Sub => {
                    let right = self.pop_number()?;
                    let left = self.pop_number()?;
                    self.stack.push(Value::Number(left - right));
                }
                
                Opcode::Mul => {
                    let right = self.pop_number()?;
                    let left = self.pop_number()?;
                    self.stack.push(Value::Number(left * right));
                }
                
                Opcode::Div => {
                    let right = self.pop_number()?;
                    let left = self.pop_number()?;
                    if right == 0.0 {
                        return Err(AetherError::RuntimeError("Division by zero".to_string()));
                    }
                    self.stack.push(Value::Number(left / right));
                }
                
                Opcode::Power => {
                    let right = self.pop_number()?;
                    let left = self.pop_number()?;
                    self.stack.push(Value::Number(left.powf(right)));
                }
                
                Opcode::Root => {
                    let value = self.pop_number()?;
                    self.stack.push(Value::Number(value.sqrt()));
                }
                
                Opcode::Equal => {
                    let right = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.stack.push(Value::Boolean(left == right));
                }
                
                Opcode::NotEqual => {
                    let right = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.stack.push(Value::Boolean(left != right));
                }
                
                Opcode::LessThan => {
                    let right = self.pop_number()?;
                    let left = self.pop_number()?;
                    self.stack.push(Value::Boolean(left < right));
                }
                
                Opcode::GreaterThan => {
                    let right = self.pop_number()?;
                    let left = self.pop_number()?;
                    self.stack.push(Value::Boolean(left > right));
                }
                
                Opcode::Approx => {
                    let right = self.pop_number()?;
                    let left = self.pop_number()?;
                    self.stack.push(Value::Boolean((left - right).abs() < APPROX_EPSILON));
                }
                
                Opcode::And => {
                    let right = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.stack.push(Value::Boolean(left.is_truthy() && right.is_truthy()));
                }
                
                Opcode::Or => {
                    let right = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.stack.push(Value::Boolean(left.is_truthy() || right.is_truthy()));
                }
                
                Opcode::Not => {
                    let value = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.stack.push(Value::Boolean(!value.is_truthy()));
                }
                
                Opcode::Input => {
                    // For VM, we'll push null as placeholder
                    self.stack.push(Value::Null);
                }
                
                Opcode::Output => {
                    let value = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    println!("Output: {:?}", value);
                    self.stack.push(value);
                }
                
                Opcode::JsonParse => {
                    let value = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    // Simplified JSON parsing
                    self.stack.push(value);
                }
                
                Opcode::Persist => {
                    let value = self.stack.last()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    println!("Persisting: {:?}", value);
                }
                
                Opcode::Query => {
                    // Placeholder for query operation
                    let value = self.stack.last()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?
                        .clone();
                    self.stack.push(value);
                }
                
                Opcode::Jump => {
                    let target = self.read_u32()? as usize;
                    self.pc = target;
                }
                
                Opcode::JumpIfFalse => {
                    let target = self.read_u32()? as usize;
                    let value = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    if !value.is_truthy() {
                        self.pc = target;
                    }
                }
                
                Opcode::JumpIfNull => {
                    let target = self.read_u32()? as usize;
                    let value = self.stack.last()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    if value.is_null() {
                        self.pc = target;
                    }
                }
                
                Opcode::Call => {
                    let _func_idx = self.read_u32()?;
                    let _arg_count = self.read_u8()?;
                    self.call_stack.push(self.pc);
                    // Function call implementation would go here
                }
                
                Opcode::Return => {
                    if let Some(return_addr) = self.call_stack.pop() {
                        self.pc = return_addr;
                    } else {
                        // End of program
                        break;
                    }
                }
                
                Opcode::Halt => {
                    let error_value = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    return Err(AetherError::RuntimeError(format!("Halted with: {:?}", error_value)));
                }
                
                Opcode::MakeArray => {
                    let count = self.read_u32()? as usize;
                    let mut elements = Vec::with_capacity(count);
                    for _ in 0..count {
                        elements.push(self.stack.pop()
                            .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?);
                    }
                    elements.reverse();
                    self.stack.push(Value::Array(elements));
                }
                
                Opcode::MakeObject => {
                    let count = self.read_u32()? as usize;
                    let mut obj = HashMap::new();
                    for _ in 0..count {
                        let value = self.stack.pop()
                            .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                        let key = self.stack.pop()
                            .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                        if let Value::String(key_str) = key {
                            obj.insert(key_str, value);
                        }
                    }
                    self.stack.push(Value::Object(obj));
                }
                
                Opcode::LoopStart => {
                    let _end_pos = self.read_u32()?;
                    // Loop start marker
                }
                
                Opcode::LoopEnd => {
                    let start_pos = self.read_u32()? as usize;
                    self.pc = start_pos;
                }
                
                Opcode::ForEach => {
                    let _var_idx = self.read_u32()?;
                    // ForEach implementation would iterate over collection
                    // For now, simplified
                }
                
                Opcode::Hash => {
                    let value = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    
                    let mut hasher = Sha256::new();
                    let input = format!("{:?}", value);
                    hasher.update(input.as_bytes());
                    let result = hasher.finalize();
                    let hash_str = format!("{:x}", result);
                    
                    self.stack.push(Value::String(hash_str));
                }
                
                Opcode::DateTime => {
                    let now = Utc::now();
                    self.stack.push(Value::String(now.to_rfc3339()));
                }
                
                Opcode::Random => {
                    let mut rng = rand::thread_rng();
                    let value: f64 = rng.gen();
                    self.stack.push(Value::Number(value));
                }
                
                Opcode::Log => {
                    let message = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    println!("[LOG] {:?}", message);
                    self.stack.push(message);
                }
                
                Opcode::Debug => {
                    println!("[DEBUG] Stack: {:?}", self.stack);
                    println!("[DEBUG] Variables: {:?}", self.variables);
                    println!("[DEBUG] PC: {}", self.pc);
                }
                
                Opcode::TestStart => {
                    let idx = self.read_u32()? as usize;
                    let name = self.program.constants.get(idx)
                        .ok_or_else(|| AetherError::RuntimeError(
                            format!("Invalid constant index: {}", idx)
                        ))?;
                    println!("[TEST] Starting test: {}", name);
                }
                
                Opcode::Assert => {
                    let condition = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    if !condition.is_truthy() {
                        return Err(AetherError::RuntimeError("Assertion failed".to_string()));
                    }
                }
                
                Opcode::Split => {
                    // Stack: [target, delimiter] (delimiter on top)
                    let delimiter = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let target = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    
                    match (target, delimiter) {
                        (Value::String(s), Value::String(delim)) => {
                            let parts: Vec<Value> = s.split(&delim).map(|p| Value::String(p.to_string())).collect();
                            self.stack.push(Value::Array(parts));
                        }
                        _ => {
                            return Err(AetherError::RuntimeError("Split requires string inputs".to_string()));
                        }
                    }
                }
                
                Opcode::Join => {
                    // Stack: [elements, separator] (separator on top)
                    let separator = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let elements = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    
                    match (elements, separator) {
                        (Value::Array(arr), Value::String(sep)) => {
                            let strings: Vec<String> = arr.iter().map(|v| match v {
                                Value::String(s) => s.clone(),
                                Value::Number(n) => n.to_string(),
                                Value::Boolean(b) => b.to_string(),
                                _ => format!("{:?}", v),
                            }).collect();
                            self.stack.push(Value::String(strings.join(&sep)));
                        }
                        _ => {
                            return Err(AetherError::RuntimeError("Join requires array and string".to_string()));
                        }
                    }
                }
                
                Opcode::Filter => {
                    // Simplified filter - just passes through for now
                    // Full implementation would need closure/predicate support
                }
                
                Opcode::Reduce => {
                    // Simplified reduce - just passes through for now  
                    // Full implementation would need closure/operation support
                }
                
                Opcode::Regex => {
                    // Stack: [pattern, target] (target on top)
                    let target = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let pattern = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    
                    match (pattern, target) {
                        (Value::String(pat), Value::String(tgt)) => {
                            match regex::Regex::new(&pat) {
                                Ok(re) => {
                                    let is_match = re.is_match(&tgt);
                                    self.stack.push(Value::Boolean(is_match));
                                }
                                Err(_) => {
                                    return Err(AetherError::RuntimeError("Invalid regex pattern".to_string()));
                                }
                            }
                        }
                        _ => {
                            return Err(AetherError::RuntimeError("Regex requires string inputs".to_string()));
                        }
                    }
                }
                
                Opcode::Encrypt => {
                    // Simplified - just return encrypted marker
                    let _key = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let data = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.stack.push(Value::String(format!("[ENCRYPTED:{:?}]", data)));
                }
                
                Opcode::Decrypt => {
                    // Simplified - just return decrypted marker
                    let _key = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let data = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    self.stack.push(Value::String(format!("[DECRYPTED:{:?}]", data)));
                }
                
                Opcode::TryStart => {
                    let _rescue_offset = self.read_u32()?;
                    // Mark try block start - actual exception handling would need more infrastructure
                }
                
                Opcode::TryEnd => {
                    // Mark try block end
                }
                
                Opcode::Retry => {
                    let _max_attempts = self.read_u8()?;
                    // Retry logic would need more infrastructure for loop control
                }
                
                Opcode::Async => {
                    // Mark async operation - would need actual async runtime
                }
                
                Opcode::Await => {
                    // Await async result - would need actual async runtime
                }
                
                Opcode::Delta => {
                    // Delta calculation - for now just passes through value
                }
                
                Opcode::Import => {
                    let idx = self.read_u32()? as usize;
                    let _module = self.program.constants.get(idx)
                        .ok_or_else(|| AetherError::RuntimeError(
                            format!("Invalid constant index: {}", idx)
                        ))?;
                    // Module import would need module system
                    self.stack.push(Value::Null);
                }
                
                Opcode::Mock => {
                    let _target_idx = self.read_u32()?;
                    // Mocking would need test infrastructure
                }
                
                Opcode::BenchmarkStart => {
                    // Store start time in a variable or state
                }
                
                Opcode::BenchmarkEnd => {
                    // Calculate and output elapsed time
                }
                
                Opcode::FileHandle => {
                    let path = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    // Return file handle placeholder
                    self.stack.push(Value::String(format!("[FILE:{:?}]", path)));
                }
                
                Opcode::FileRead => {
                    let _handle = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    // Read file content - simplified
                    self.stack.push(Value::String("[FILE CONTENT]".to_string()));
                }
                
                Opcode::FileWrite => {
                    let content = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let _handle = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    // Write to file - simplified
                    println!("[FILE WRITE] {:?}", content);
                }
                
                Opcode::FileAppend => {
                    let content = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    let _handle = self.stack.pop()
                        .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
                    // Append to file - simplified
                    println!("[FILE APPEND] {:?}", content);
                }
                
                Opcode::End => {
                    break;
                }
                
                _ => {
                    return Err(AetherError::RuntimeError(format!(
                        "Unimplemented opcode: {:?}",
                        opcode
                    )));
                }
            }
        }
        
        // Return top of stack or null
        Ok(self.stack.pop().unwrap_or(Value::Null))
    }
    
    /// Read a u8 from bytecode
    fn read_u8(&mut self) -> Result<u8> {
        if self.pc >= self.program.code.len() {
            return Err(AetherError::RuntimeError("Unexpected end of bytecode".to_string()));
        }
        let value = self.program.code[self.pc];
        self.pc += 1;
        Ok(value)
    }
    
    /// Read a u32 from bytecode (big-endian)
    fn read_u32(&mut self) -> Result<u32> {
        if self.pc + 4 > self.program.code.len() {
            return Err(AetherError::RuntimeError("Unexpected end of bytecode".to_string()));
        }
        let bytes = &self.program.code[self.pc..self.pc + 4];
        self.pc += 4;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    
    /// Read a f64 from bytecode
    fn read_f64(&mut self) -> Result<f64> {
        if self.pc + 8 > self.program.code.len() {
            return Err(AetherError::RuntimeError("Unexpected end of bytecode".to_string()));
        }
        let bytes = &self.program.code[self.pc..self.pc + 8];
        self.pc += 8;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(f64::from_be_bytes(array))
    }
    
    /// Pop a number from stack
    fn pop_number(&mut self) -> Result<f64> {
        let value = self.stack.pop()
            .ok_or_else(|| AetherError::RuntimeError("Stack underflow".to_string()))?;
        match value {
            Value::Number(n) => Ok(n),
            _ => Err(AetherError::RuntimeError(format!(
                "Expected number, got {:?}",
                value
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::Opcode;
    
    #[test]
    fn test_vm_push_number() {
        let mut program = BytecodeProgram::new();
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(42.0);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::Number(42.0));
    }
    
    #[test]
    fn test_vm_arithmetic() {
        let mut program = BytecodeProgram::new();
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(10.0);
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(5.0);
        program.emit_opcode(Opcode::Add);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::Number(15.0));
    }
    
    #[test]
    fn test_vm_variable() {
        let mut program = BytecodeProgram::new();
        let var_idx = program.add_constant("x".to_string());
        
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(100.0);
        program.emit_opcode(Opcode::StoreVar);
        program.emit_u32(var_idx);
        program.emit_opcode(Opcode::LoadVar);
        program.emit_u32(var_idx);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::Number(100.0));
    }
    
    #[test]
    fn test_vm_output() {
        let mut program = BytecodeProgram::new();
        let str_idx = program.add_constant("Hello, VM!".to_string());
        
        program.emit_opcode(Opcode::PushString);
        program.emit_u32(str_idx);
        program.emit_opcode(Opcode::Output);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::String("Hello, VM!".to_string()));
    }
    
    #[test]
    fn test_vm_power() {
        let mut program = BytecodeProgram::new();
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(2.0);
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(3.0);
        program.emit_opcode(Opcode::Power);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::Number(8.0));
    }
    
    #[test]
    fn test_vm_root() {
        let mut program = BytecodeProgram::new();
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(16.0);
        program.emit_opcode(Opcode::Root);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::Number(4.0));
    }
    
    #[test]
    fn test_vm_split() {
        let mut program = BytecodeProgram::new();
        let str_idx = program.add_constant("a,b,c".to_string());
        let delim_idx = program.add_constant(",".to_string());
        
        program.emit_opcode(Opcode::PushString);
        program.emit_u32(str_idx);
        program.emit_opcode(Opcode::PushString);
        program.emit_u32(delim_idx);
        program.emit_opcode(Opcode::Split);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        match result {
            Value::Array(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], Value::String("a".to_string()));
                assert_eq!(items[1], Value::String("b".to_string()));
                assert_eq!(items[2], Value::String("c".to_string()));
            }
            _ => panic!("Expected array result"),
        }
    }
    
    #[test]
    fn test_vm_equal() {
        let mut program = BytecodeProgram::new();
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(5.0);
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(5.0);
        program.emit_opcode(Opcode::Equal);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::Boolean(true));
    }
    
    #[test]
    fn test_vm_not_equal() {
        let mut program = BytecodeProgram::new();
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(5.0);
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(3.0);
        program.emit_opcode(Opcode::NotEqual);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::Boolean(true));
    }
    
    #[test]
    fn test_vm_comparison() {
        let mut program = BytecodeProgram::new();
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(10.0);
        program.emit_opcode(Opcode::PushNumber);
        program.emit_f64(5.0);
        program.emit_opcode(Opcode::GreaterThan);
        program.emit_opcode(Opcode::End);
        
        let mut vm = VM::new(program);
        let result = vm.execute().unwrap();
        
        assert_eq!(result, Value::Boolean(true));
    }
}
