//! Compiler for converting AST to bytecode

use crate::bytecode::{BytecodeProgram, Opcode};
use crate::constants::PIPE_VARIABLE;
use crate::error::{AetherError, Result};
use crate::parser::{AstNode, LiteralValue};
use std::collections::HashMap;

/// Compiler context
pub struct Compiler {
    program: BytecodeProgram,
    functions: HashMap<String, usize>,
    loop_stack: Vec<LoopContext>,
}

#[allow(dead_code)]
struct LoopContext {
    _start_pos: usize,
    end_patches: Vec<usize>,
}

impl Compiler {
    /// Create a new compiler
    pub fn new() -> Self {
        Compiler {
            program: BytecodeProgram::new(),
            functions: HashMap::new(),
            loop_stack: Vec::new(),
        }
    }
    
    /// Compile AST nodes to bytecode
    pub fn compile(&mut self, nodes: Vec<AstNode>) -> Result<BytecodeProgram> {
        for node in nodes {
            self.compile_node(&node)?;
        }
        
        // Emit end instruction
        self.program.emit_opcode(Opcode::End);
        
        Ok(self.program.clone())
    }
    
    /// Compile a single AST node
    fn compile_node(&mut self, node: &AstNode) -> Result<()> {
        match node {
            AstNode::Literal(lit) => self.compile_literal(lit)?,
            
            AstNode::Variable(name) => {
                // Special handling for _pipe variable used in pipe operations
                // In bytecode, the piped value is already on the stack
                if name == PIPE_VARIABLE {
                    // Don't emit LoadVar - value is already on stack from Pipe source
                    // No-op: the value is already where it needs to be
                } else {
                    let idx = self.program.add_constant(name.clone());
                    self.program.emit_opcode(Opcode::LoadVar);
                    self.program.emit_u32(idx);
                }
            }
            
            AstNode::Input => {
                self.program.emit_opcode(Opcode::Input);
            }
            
            AstNode::Output(value) => {
                self.compile_node(value)?;
                self.program.emit_opcode(Opcode::Output);
            }
            
            AstNode::Sequence(nodes) => {
                for node in nodes {
                    self.compile_node(node)?;
                }
            }
            
            AstNode::Pipe { source, operation } => {
                self.compile_node(source)?;
                self.compile_node(operation)?;
            }
            
            AstNode::PipeInto { value, variable } => {
                self.compile_node(value)?;
                self.program.emit_opcode(Opcode::Dup);
                let idx = self.program.add_constant(variable.clone());
                self.program.emit_opcode(Opcode::StoreVar);
                self.program.emit_u32(idx);
            }
            
            AstNode::JsonParse(value) => {
                self.compile_node(value)?;
                self.program.emit_opcode(Opcode::JsonParse);
            }
            
            AstNode::Persist(value) => {
                self.compile_node(value)?;
                self.program.emit_opcode(Opcode::Persist);
            }
            
            AstNode::Guard { condition, then_branch } => {
                self.compile_node(condition)?;
                self.program.emit_opcode(Opcode::JumpIfNull);
                let jump_pos = self.program.position();
                self.program.emit_u32(0); // Placeholder
                
                // Compile then branch
                self.compile_node(then_branch)?;
                
                // Patch jump
                let end_pos = self.program.position();
                self.program.patch_u32(jump_pos, end_pos as u32);
            }
            
            AstNode::Halt(value) => {
                self.compile_node(value)?;
                self.program.emit_opcode(Opcode::Halt);
            }
            
            AstNode::IfThen { condition, then_branch, else_branch } => {
                self.compile_node(condition)?;
                self.program.emit_opcode(Opcode::JumpIfFalse);
                let jump_else_pos = self.program.position();
                self.program.emit_u32(0); // Placeholder
                
                // Compile then branch
                self.compile_node(then_branch)?;
                
                if let Some(else_node) = else_branch {
                    // Jump over else branch
                    self.program.emit_opcode(Opcode::Jump);
                    let jump_end_pos = self.program.position();
                    self.program.emit_u32(0); // Placeholder
                    
                    // Patch else jump
                    let else_start = self.program.position();
                    self.program.patch_u32(jump_else_pos, else_start as u32);
                    
                    // Compile else branch
                    self.compile_node(else_node)?;
                    
                    // Patch end jump
                    let end_pos = self.program.position();
                    self.program.patch_u32(jump_end_pos, end_pos as u32);
                } else {
                    // Patch else jump to end
                    let end_pos = self.program.position();
                    self.program.patch_u32(jump_else_pos, end_pos as u32);
                }
            }
            
            AstNode::Loop { condition, body } => {
                let start_pos = self.program.position();
                self.program.emit_opcode(Opcode::LoopStart);
                let end_patch_pos = self.program.position();
                self.program.emit_u32(0); // Placeholder for end position
                
                self.loop_stack.push(LoopContext {
                    _start_pos: start_pos,
                    end_patches: vec![end_patch_pos],
                });
                
                // Compile condition if present
                if let Some(cond) = condition {
                    self.compile_node(cond)?;
                    self.program.emit_opcode(Opcode::JumpIfFalse);
                    let cond_jump_pos = self.program.position();
                    self.program.emit_u32(0); // Placeholder
                    
                    if let Some(ctx) = self.loop_stack.last_mut() {
                        ctx.end_patches.push(cond_jump_pos);
                    }
                }
                
                // Compile body
                self.compile_node(body)?;
                
                // Jump back to start
                self.program.emit_opcode(Opcode::LoopEnd);
                self.program.emit_u32(start_pos as u32);
                
                // Patch all end jumps
                let end_pos = self.program.position();
                if let Some(ctx) = self.loop_stack.pop() {
                    for patch_pos in ctx.end_patches {
                        self.program.patch_u32(patch_pos, end_pos as u32);
                    }
                }
            }
            
            AstNode::ForEach { variable, collection, body } => {
                self.compile_node(collection)?;
                let var_idx = self.program.add_constant(variable.clone());
                self.program.emit_opcode(Opcode::ForEach);
                self.program.emit_u32(var_idx);
                
                let start_pos = self.program.position();
                self.compile_node(body)?;
                
                // Jump back for next iteration (handled by VM)
                self.program.emit_opcode(Opcode::LoopEnd);
                self.program.emit_u32(start_pos as u32);
            }
            
            AstNode::Function { name, body } => {
                let func_idx = self.program.add_constant(name.clone());
                self.functions.insert(name.clone(), self.program.position());
                
                // Jump over function body
                self.program.emit_opcode(Opcode::Jump);
                let jump_pos = self.program.position();
                self.program.emit_u32(0); // Placeholder
                
                // Function body
                let func_start = self.program.position();
                self.compile_node(body)?;
                self.program.emit_opcode(Opcode::Return);
                
                // Patch jump
                let end_pos = self.program.position();
                self.program.patch_u32(jump_pos, end_pos as u32);
                
                // Store function location
                self.program.emit_opcode(Opcode::PushNumber);
                self.program.emit_f64(func_start as f64);
                self.program.emit_opcode(Opcode::StoreVar);
                self.program.emit_u32(func_idx);
            }
            
            AstNode::TryRescue { try_body, rescue_body } => {
                self.program.emit_opcode(Opcode::TryStart);
                let rescue_pos = self.program.position();
                self.program.emit_u32(0); // Placeholder
                
                self.compile_node(try_body)?;
                self.program.emit_opcode(Opcode::TryEnd);
                
                if let Some(rescue) = rescue_body {
                    let rescue_start = self.program.position();
                    self.program.patch_u32(rescue_pos, rescue_start as u32);
                    self.compile_node(rescue)?;
                }
            }
            
            AstNode::Retry { max_attempts, body } => {
                let attempts = if let Some(a) = max_attempts {
                    *a as f64
                } else {
                    3.0 // default
                };
                self.emit_number(attempts);
                self.program.emit_opcode(Opcode::Retry);
                self.compile_node(body)?;
            }
            
            AstNode::Filter { predicate } => {
                self.compile_node(predicate)?;
                self.program.emit_opcode(Opcode::Filter);
            }
            
            AstNode::Reduce { operation, initial } => {
                self.compile_node(initial)?;
                self.compile_node(operation)?;
                self.program.emit_opcode(Opcode::Reduce);
            }
            
            AstNode::Power { base, exponent } => {
                self.compile_node(base)?;
                self.compile_node(exponent)?;
                self.program.emit_opcode(Opcode::Power);
            }
            
            AstNode::Root { value } => {
                self.compile_node(value)?;
                self.program.emit_opcode(Opcode::Root);
            }
            
            AstNode::Equal { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::Equal);
            }
            
            AstNode::NotEqual { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::NotEqual);
            }
            
            AstNode::And { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::And);
            }
            
            AstNode::Or { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::Or);
            }
            
            AstNode::Not { operand } => {
                self.compile_node(operand)?;
                self.program.emit_opcode(Opcode::Not);
            }
            
            AstNode::Approx { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::Approx);
            }
            
            AstNode::Comparison { left, operator, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                
                use crate::parser::ComparisonOp;
                match operator {
                    ComparisonOp::GreaterThan => self.program.emit_opcode(Opcode::GreaterThan),
                    ComparisonOp::LessThan => self.program.emit_opcode(Opcode::LessThan),
                    ComparisonOp::GreaterEqual => self.program.emit_opcode(Opcode::GreaterEqual),
                    ComparisonOp::LessEqual => self.program.emit_opcode(Opcode::LessEqual),
                }
            }
            
            AstNode::Split { target, delimiter } => {
                // If target is Empty, it means we're using the piped value (already on stack)
                // Otherwise, compile the target normally
                if !matches!(target.as_ref(), AstNode::Empty) {
                    self.compile_node(target)?;
                }
                // If delimiter is provided, compile it; otherwise use default (space)
                if let Some(delim) = delimiter {
                    self.compile_node(delim)?;
                } else {
                    // Push default delimiter (space, matching runtime behavior)
                    self.compile_literal(&LiteralValue::String(" ".to_string()))?;
                }
                self.program.emit_opcode(Opcode::Split);
            }
            
            AstNode::Join { elements, separator } => {
                // If elements is Empty, it means we're using the piped value (already on stack)
                if !matches!(elements.as_ref(), AstNode::Empty) {
                    self.compile_node(elements)?;
                }
                // If separator is provided, compile it; otherwise use default (empty string)
                if let Some(sep) = separator {
                    self.compile_node(sep)?;
                } else {
                    // Push default separator (empty string, matching runtime behavior)
                    self.compile_literal(&LiteralValue::String("".to_string()))?;
                }
                self.program.emit_opcode(Opcode::Join);
            }
            
            AstNode::Hash { data } => {
                self.compile_node(data)?;
                self.program.emit_opcode(Opcode::Hash);
            }
            
            AstNode::Encrypt { data, key } => {
                self.compile_node(data)?;
                self.compile_node(key)?;
                self.program.emit_opcode(Opcode::Encrypt);
            }
            
            AstNode::Decrypt { data, key } => {
                self.compile_node(data)?;
                self.compile_node(key)?;
                self.program.emit_opcode(Opcode::Decrypt);
            }
            
            AstNode::Async { body } => {
                self.program.emit_opcode(Opcode::Async);
                self.compile_node(body)?;
            }
            
            AstNode::Await { expression } => {
                self.compile_node(expression)?;
                self.program.emit_opcode(Opcode::Await);
            }
            
            AstNode::DateTime => {
                self.program.emit_opcode(Opcode::DateTime);
            }
            
            AstNode::Random => {
                self.program.emit_opcode(Opcode::Random);
            }
            
            AstNode::Delta { name, value } => {
                self.compile_node(value)?;
                let idx = self.program.add_constant(name.clone());
                self.program.emit_opcode(Opcode::StoreVar);
                self.program.emit_u32(idx);
                self.program.emit_opcode(Opcode::Delta);
            }
            
            AstNode::Import { module } => {
                let idx = self.program.add_constant(module.clone());
                self.program.emit_opcode(Opcode::Import);
                self.program.emit_u32(idx);
            }
            
            AstNode::Log { message } => {
                self.compile_node(message)?;
                self.program.emit_opcode(Opcode::Log);
            }
            
            AstNode::Debug => {
                self.program.emit_opcode(Opcode::Debug);
            }
            
            AstNode::Test { name, body } => {
                let idx = self.program.add_constant(name.clone());
                self.program.emit_opcode(Opcode::TestStart);
                self.program.emit_u32(idx);
                self.compile_node(body)?;
            }
            
            AstNode::Assert { condition } => {
                self.compile_node(condition)?;
                self.program.emit_opcode(Opcode::Assert);
            }
            
            AstNode::Mock { target } => {
                // Compile the target node to get its representation
                self.compile_node(target)?;
                // For now, emit with a default index; full implementation would need target resolution
                self.program.emit_opcode(Opcode::Mock);
                self.program.emit_u32(0);
            }
            
            AstNode::Benchmark { body } => {
                self.program.emit_opcode(Opcode::BenchmarkStart);
                self.compile_node(body)?;
                self.program.emit_opcode(Opcode::BenchmarkEnd);
            }
            
            AstNode::FileHandle { path } => {
                self.compile_node(path)?;
                self.program.emit_opcode(Opcode::FileHandle);
            }
            
            AstNode::ReadContent { source } => {
                self.compile_node(source)?;
                self.program.emit_opcode(Opcode::FileRead);
            }
            
            AstNode::WriteContent { target, content } => {
                self.compile_node(target)?;
                self.compile_node(content)?;
                self.program.emit_opcode(Opcode::FileWrite);
            }
            
            AstNode::AppendContent { target, content } => {
                self.compile_node(target)?;
                self.compile_node(content)?;
                self.program.emit_opcode(Opcode::FileAppend);
            }
            
            AstNode::Immutable { name, value } => {
                self.compile_node(value)?;
                let idx = self.program.add_constant(name.clone());
                self.program.emit_opcode(Opcode::StoreImmutable);
                self.program.emit_u32(idx);
            }
            
            AstNode::Empty => {
                self.program.emit_opcode(Opcode::PushNull);
            }
            
            // HTTP operations
            AstNode::HttpGet { url, headers } => {
                self.compile_node(url)?;
                if let Some(h) = headers {
                    self.compile_node(h)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                self.program.emit_opcode(Opcode::HttpGet);
            }
            
            AstNode::HttpPost { url, body, headers } => {
                self.compile_node(url)?;
                if let Some(b) = body {
                    self.compile_node(b)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                if let Some(h) = headers {
                    self.compile_node(h)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                self.program.emit_opcode(Opcode::HttpPost);
            }
            
            AstNode::HttpPut { url, body, headers } => {
                self.compile_node(url)?;
                if let Some(b) = body {
                    self.compile_node(b)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                if let Some(h) = headers {
                    self.compile_node(h)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                self.program.emit_opcode(Opcode::HttpPut);
            }
            
            AstNode::HttpDelete { url, headers } => {
                self.compile_node(url)?;
                if let Some(h) = headers {
                    self.compile_node(h)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                self.program.emit_opcode(Opcode::HttpDelete);
            }
            
            AstNode::HttpPatch { url, body, headers } => {
                self.compile_node(url)?;
                if let Some(b) = body {
                    self.compile_node(b)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                if let Some(h) = headers {
                    self.compile_node(h)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                self.program.emit_opcode(Opcode::HttpPatch);
            }
            
            AstNode::HttpHead { url, headers } => {
                self.compile_node(url)?;
                if let Some(h) = headers {
                    self.compile_node(h)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                self.program.emit_opcode(Opcode::HttpHead);
            }
            
            AstNode::HttpOptions { url, headers } => {
                self.compile_node(url)?;
                if let Some(h) = headers {
                    self.compile_node(h)?;
                } else {
                    self.program.emit_opcode(Opcode::PushNull);
                }
                self.program.emit_opcode(Opcode::HttpOptions);
            }
            
            // File system operations
            AstNode::Directory { path } => {
                self.compile_node(path)?;
                self.program.emit_opcode(Opcode::Directory);
            }
            
            AstNode::PathResolve { path } => {
                self.compile_node(path)?;
                self.program.emit_opcode(Opcode::PathResolve);
            }
            
            AstNode::DeleteFile { target } => {
                self.compile_node(target)?;
                self.program.emit_opcode(Opcode::DeleteFile);
            }
            
            AstNode::SetPermission { target, permission } => {
                self.compile_node(target)?;
                self.compile_node(permission)?;
                self.program.emit_opcode(Opcode::SetPermission);
            }
            
            // Process & Environment operations
            AstNode::EnvVar { name } => {
                self.compile_node(name)?;
                self.program.emit_opcode(Opcode::EnvVar);
            }
            
            AstNode::ProcessCreate { command } => {
                self.compile_node(command)?;
                self.program.emit_opcode(Opcode::ProcessCreate);
            }
            
            AstNode::ShellExec { command } => {
                self.compile_node(command)?;
                self.program.emit_opcode(Opcode::ShellExec);
            }
            
            AstNode::MemoryAlloc { size } => {
                self.compile_node(size)?;
                self.program.emit_opcode(Opcode::MemoryAlloc);
            }
            
            AstNode::ExitProgram { code } => {
                self.compile_node(code)?;
                self.program.emit_opcode(Opcode::ExitProgram);
            }
            
            AstNode::SendSignal { signal, target } => {
                self.compile_node(signal)?;
                self.compile_node(target)?;
                self.program.emit_opcode(Opcode::SendSignal);
            }
            
            // Networking operations
            AstNode::CreateSocket { socket_type } => {
                self.compile_node(socket_type)?;
                self.program.emit_opcode(Opcode::CreateSocket);
            }
            
            AstNode::ListenPort { port } => {
                self.compile_node(port)?;
                self.program.emit_opcode(Opcode::ListenPort);
            }
            
            AstNode::ConnectRemote { address } => {
                self.compile_node(address)?;
                self.program.emit_opcode(Opcode::ConnectRemote);
            }
            
            AstNode::PortNumber { number } => {
                self.compile_node(number)?;
                self.program.emit_opcode(Opcode::PortNumber);
            }
            
            AstNode::CreatePacket { data } => {
                self.compile_node(data)?;
                self.program.emit_opcode(Opcode::CreatePacket);
            }
            
            AstNode::Handshake { connection } => {
                self.compile_node(connection)?;
                self.program.emit_opcode(Opcode::Handshake);
            }
            
            // Security operations
            AstNode::Sign { data, key } => {
                self.compile_node(data)?;
                self.compile_node(key)?;
                self.program.emit_opcode(Opcode::Sign);
            }
            
            AstNode::VerifySignature { signature, data, key } => {
                self.compile_node(signature)?;
                self.compile_node(data)?;
                self.compile_node(key)?;
                self.program.emit_opcode(Opcode::VerifySignature);
            }
            
            // Stream & Buffer operations
            AstNode::CreateStream { source } => {
                self.compile_node(source)?;
                self.program.emit_opcode(Opcode::CreateStream);
            }
            
            AstNode::CreateBuffer { size } => {
                self.compile_node(size)?;
                self.program.emit_opcode(Opcode::CreateBuffer);
            }
            
            AstNode::FlushBuffer { target } => {
                self.compile_node(target)?;
                self.program.emit_opcode(Opcode::FlushBuffer);
            }
            
            AstNode::EndOfFile => {
                self.program.emit_opcode(Opcode::EndOfFile);
            }
            
            AstNode::SkipBytes { source, count } => {
                self.compile_node(source)?;
                self.compile_node(count)?;
                self.program.emit_opcode(Opcode::SkipBytes);
            }
            
            // Concurrency operations
            AstNode::Thread { body } => {
                self.program.emit_opcode(Opcode::Thread);
                self.compile_node(body)?;
            }
            
            AstNode::Lock { body } => {
                self.program.emit_opcode(Opcode::Lock);
                self.compile_node(body)?;
            }
            
            AstNode::Emit { event } => {
                self.compile_node(event)?;
                self.program.emit_opcode(Opcode::Emit);
            }
            
            AstNode::Watch { event, handler } => {
                self.compile_node(event)?;
                self.compile_node(handler)?;
                self.program.emit_opcode(Opcode::Watch);
            }
            
            // Data operations
            AstNode::RegexMatch { pattern, target } => {
                self.compile_node(pattern)?;
                self.compile_node(target)?;
                self.program.emit_opcode(Opcode::RegexMatch);
            }
            
            AstNode::Auth { token } => {
                self.compile_node(token)?;
                self.program.emit_opcode(Opcode::Auth);
            }
            
            AstNode::PropertyAccess { object, property } => {
                self.compile_node(object)?;
                let idx = self.program.add_constant(property.clone());
                self.program.emit_opcode(Opcode::PushString);
                self.program.emit_u32(idx);
                self.program.emit_opcode(Opcode::PropertyAccess);
            }
            
            // Math operations
            AstNode::Infinity => {
                self.program.emit_opcode(Opcode::Infinity);
            }
            
            // Bootstrap operations - arithmetic
            AstNode::Add { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::Add);
            }
            
            AstNode::Subtract { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::Sub);
            }
            
            AstNode::Multiply { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::Mul);
            }
            
            AstNode::Divide { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                self.program.emit_opcode(Opcode::Div);
            }
            
            AstNode::Modulo { left, right } => {
                // Note: Modulo not in bytecode, use runtime
                self.compile_node(left)?;
                self.compile_node(right)?;
                // Emit as custom handling - will need to add to bytecode
                return Err(AetherError::CompilerError(
                    "Modulo not yet supported in bytecode compilation".to_string()
                ));
            }
            
            AstNode::StringConcat { left, right } => {
                self.compile_node(left)?;
                self.compile_node(right)?;
                // String concat needs special handling in bytecode
                // For now, emit as Join with empty separator
                self.program.emit_opcode(Opcode::Join);
            }
            
            AstNode::Length { value } => {
                self.compile_node(value)?;
                // Length needs bytecode support
                return Err(AetherError::CompilerError(
                    "Length not yet supported in bytecode compilation".to_string()
                ));
            }
            
            AstNode::Index { target, index } => {
                self.compile_node(target)?;
                self.compile_node(index)?;
                // Index needs bytecode support
                return Err(AetherError::CompilerError(
                    "Index not yet supported in bytecode compilation".to_string()
                ));
            }
            
            AstNode::ArrayLiteral { elements } => {
                for elem in elements {
                    self.compile_node(elem)?;
                }
                self.program.emit_opcode(Opcode::MakeArray);
                self.program.emit_u32(elements.len() as u32);
            }
            
            AstNode::ObjectLiteral { pairs } => {
                for (key, value) in pairs {
                    let key_idx = self.program.add_constant(key.clone());
                    self.program.emit_opcode(Opcode::PushString);
                    self.program.emit_u32(key_idx);
                    self.compile_node(value)?;
                }
                self.program.emit_opcode(Opcode::MakeObject);
                self.program.emit_u32(pairs.len() as u32);
            }
            
            _ => {
                return Err(AetherError::CompilerError(format!(
                    "Unsupported AST node: {:?}",
                    node
                )));
            }
        }
        
        Ok(())
    }
    
    /// Compile a literal value
    fn compile_literal(&mut self, lit: &LiteralValue) -> Result<()> {
        match lit {
            LiteralValue::String(s) => {
                let idx = self.program.add_constant(s.clone());
                self.program.emit_opcode(Opcode::PushString);
                self.program.emit_u32(idx);
            }
            LiteralValue::Number(n) => {
                self.emit_number(*n);
            }
        }
        Ok(())
    }
    
    /// Helper to emit a number
    fn emit_number(&mut self, value: f64) {
        self.program.emit_opcode(Opcode::PushNumber);
        self.program.emit_f64(value);
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    
    #[test]
    fn test_compile_simple_output() {
        let source = "📤 \"Hello\"".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut compiler = Compiler::new();
        let program = compiler.compile(ast).unwrap();
        
        // Should have "Hello" in constants
        assert_eq!(program.constants.len(), 1);
        assert_eq!(program.constants[0], "Hello");
        
        // Should have bytecode
        assert!(!program.code.is_empty());
    }
    
    #[test]
    fn test_compile_literal_number() {
        let mut compiler = Compiler::new();
        compiler.compile_literal(&LiteralValue::Number(42.0)).unwrap();
        
        assert_eq!(compiler.program.code[0], Opcode::PushNumber.to_byte());
    }
    
    #[test]
    fn test_compile_variable() {
        let node = AstNode::Variable("test".to_string());
        let mut compiler = Compiler::new();
        compiler.compile_node(&node).unwrap();
        
        assert_eq!(compiler.program.code[0], Opcode::LoadVar.to_byte());
        assert_eq!(compiler.program.constants[0], "test");
    }
    
    #[test]
    fn test_compile_power() {
        let source = "2 ⇢ ↑3".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut compiler = Compiler::new();
        let program = compiler.compile(ast).unwrap();
        
        // Should compile to: PushNumber 2, PushNumber 3, Power
        assert!(!program.code.is_empty());
    }
    
    #[test]
    fn test_compile_root() {
        let source = "16 ⇢ √".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut compiler = Compiler::new();
        let program = compiler.compile(ast).unwrap();
        
        // Should compile without error
        assert!(!program.code.is_empty());
    }
    
    #[test]
    fn test_compile_split() {
        let source = "\"a,b,c\" ⇢ ✂\",\"".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut compiler = Compiler::new();
        let program = compiler.compile(ast).unwrap();
        
        // Should have two constants: the string and delimiter
        assert_eq!(program.constants.len(), 2);
        assert!(!program.code.is_empty());
    }
    
    #[test]
    fn test_compile_pipe_into() {
        let source = "\"test\" ▷ x".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut compiler = Compiler::new();
        let program = compiler.compile(ast).unwrap();
        
        // Should have variable name in constants
        assert!(program.constants.contains(&"x".to_string()));
    }
}
