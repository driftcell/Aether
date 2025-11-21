//! Bytecode format and serialization for Aether
//!
//! This module defines the bytecode instruction set and provides
//! serialization/deserialization for .aeb (Aether Bytecode) files.

use crate::error::{AetherError, Result};
use std::io::{Read, Write};

/// Magic number for .aeb files: "AEB\0"
pub const MAGIC_NUMBER: [u8; 4] = [0x41, 0x45, 0x42, 0x00];

/// Bytecode format version
pub const VERSION: u8 = 1;

/// Bytecode instruction opcodes
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    // Stack operations
    /// Push null onto stack
    PushNull,
    /// Push boolean onto stack (followed by 1 byte: 0=false, 1=true)
    PushBool,
    /// Push number onto stack (followed by 8 bytes: f64)
    PushNumber,
    /// Push string from constant pool (followed by 4 bytes: index)
    PushString,
    /// Pop value from stack
    Pop,
    /// Duplicate top value on stack
    Dup,
    
    // Variable operations
    /// Load variable (followed by 4 bytes: name index in constant pool)
    LoadVar,
    /// Store variable (followed by 4 bytes: name index in constant pool)
    StoreVar,
    /// Store immutable variable (followed by 4 bytes: name index)
    StoreImmutable,
    
    // Arithmetic operations
    /// Add top two values
    Add,
    /// Subtract top two values
    Sub,
    /// Multiply top two values
    Mul,
    /// Divide top two values
    Div,
    /// Power operation
    Power,
    /// Square root
    Root,
    
    // Comparison operations
    /// Equal comparison
    Equal,
    /// Not equal comparison
    NotEqual,
    /// Less than
    LessThan,
    /// Greater than
    GreaterThan,
    /// Approximate equality
    Approx,
    
    // Logical operations
    /// Logical AND
    And,
    /// Logical OR
    Or,
    /// Logical NOT
    Not,
    
    // I/O operations
    /// Read input
    Input,
    /// Output top value
    Output,
    
    // Data operations
    /// Parse JSON
    JsonParse,
    /// Persist value
    Persist,
    /// Query operation
    Query,
    
    // Control flow
    /// Jump (followed by 4 bytes: offset)
    Jump,
    /// Jump if false (followed by 4 bytes: offset)
    JumpIfFalse,
    /// Jump if null/guard (followed by 4 bytes: offset)
    JumpIfNull,
    /// Call function (followed by 4 bytes: function index, 1 byte: arg count)
    Call,
    /// Return from function
    Return,
    /// Halt with error
    Halt,
    
    // Collection operations
    /// Create array (followed by 4 bytes: element count)
    MakeArray,
    /// Create object (followed by 4 bytes: key-value pair count)
    MakeObject,
    
    // Loop operations
    /// Start loop (followed by 4 bytes: end offset)
    LoopStart,
    /// Loop end (jumps back to start)
    LoopEnd,
    
    // Advanced operations
    /// ForEach operation (followed by 4 bytes: var name index)
    ForEach,
    /// Filter operation
    Filter,
    /// Reduce operation
    Reduce,
    /// Split string/array
    Split,
    /// Join array elements
    Join,
    /// Regex match
    Regex,
    /// Hash operation
    Hash,
    /// Encrypt operation
    Encrypt,
    /// Decrypt operation
    Decrypt,
    /// Try-rescue start (followed by 4 bytes: rescue offset)
    TryStart,
    /// Try-rescue end
    TryEnd,
    /// Retry operation (followed by 1 byte: max attempts)
    Retry,
    
    // Async operations
    /// Async operation start
    Async,
    /// Await operation
    Await,
    
    // Time and random
    /// Get current datetime
    DateTime,
    /// Generate random value
    Random,
    /// Get delta/difference
    Delta,
    
    // System operations
    /// Import module (followed by 4 bytes: name index)
    Import,
    /// Log message
    Log,
    /// Debug breakpoint
    Debug,
    
    // Testing operations
    /// Start test (followed by 4 bytes: name index)
    TestStart,
    /// Assert condition
    Assert,
    /// Mock operation (followed by 4 bytes: target index)
    Mock,
    /// Benchmark start
    BenchmarkStart,
    /// Benchmark end
    BenchmarkEnd,
    
    // File system operations
    /// File handle
    FileHandle,
    /// Read from file
    FileRead,
    /// Write to file
    FileWrite,
    /// Append to file
    FileAppend,
    /// Directory operations
    Directory,
    /// Path resolution
    PathResolve,
    /// Delete file
    DeleteFile,
    /// Set permissions
    SetPermission,
    
    // HTTP operations
    /// HTTP GET request (url on stack, optional headers)
    HttpGet,
    /// HTTP POST request (url, body, optional headers on stack)
    HttpPost,
    /// HTTP PUT request (url, body, optional headers on stack)
    HttpPut,
    /// HTTP DELETE request (url, optional headers on stack)
    HttpDelete,
    /// HTTP PATCH request (url, body, optional headers on stack)
    HttpPatch,
    /// HTTP HEAD request (url, optional headers on stack)
    HttpHead,
    /// HTTP OPTIONS request (url, optional headers on stack)
    HttpOptions,
    
    // Process & Environment operations
    /// Get environment variable
    EnvVar,
    /// Create process
    ProcessCreate,
    /// Execute shell command
    ShellExec,
    /// Allocate memory
    MemoryAlloc,
    /// Exit program
    ExitProgram,
    /// Send signal
    SendSignal,
    
    // Networking operations
    /// Create socket
    CreateSocket,
    /// Listen on port
    ListenPort,
    /// Connect to remote
    ConnectRemote,
    /// Port number
    PortNumber,
    /// Create packet
    CreatePacket,
    /// Handshake
    Handshake,
    
    // Security operations
    /// Sign data
    Sign,
    /// Verify signature
    VerifySignature,
    
    // Stream & Buffer operations
    /// Create stream
    CreateStream,
    /// Create buffer
    CreateBuffer,
    /// Flush buffer
    FlushBuffer,
    /// End of file marker
    EndOfFile,
    /// Skip bytes
    SkipBytes,
    
    // Concurrency operations
    /// Create thread
    Thread,
    /// Lock/Mutex
    Lock,
    /// Emit event
    Emit,
    /// Watch/Listen event
    Watch,
    
    // Data operations
    /// Regex match
    RegexMatch,
    /// Authentication token
    Auth,
    /// Property access
    PropertyAccess,
    
    // Math operations
    /// Infinity value
    Infinity,
    
    // End of program
    End,
}

impl Opcode {
    /// Convert opcode to byte
    pub fn to_byte(&self) -> u8 {
        match self {
            Opcode::PushNull => 0x00,
            Opcode::PushBool => 0x01,
            Opcode::PushNumber => 0x02,
            Opcode::PushString => 0x03,
            Opcode::Pop => 0x04,
            Opcode::Dup => 0x05,
            
            Opcode::LoadVar => 0x10,
            Opcode::StoreVar => 0x11,
            Opcode::StoreImmutable => 0x12,
            
            Opcode::Add => 0x20,
            Opcode::Sub => 0x21,
            Opcode::Mul => 0x22,
            Opcode::Div => 0x23,
            Opcode::Power => 0x24,
            Opcode::Root => 0x25,
            
            Opcode::Equal => 0x30,
            Opcode::NotEqual => 0x31,
            Opcode::LessThan => 0x32,
            Opcode::GreaterThan => 0x33,
            Opcode::Approx => 0x34,
            
            Opcode::And => 0x40,
            Opcode::Or => 0x41,
            Opcode::Not => 0x42,
            
            Opcode::Input => 0x50,
            Opcode::Output => 0x51,
            
            Opcode::JsonParse => 0x60,
            Opcode::Persist => 0x61,
            Opcode::Query => 0x62,
            
            Opcode::Jump => 0x70,
            Opcode::JumpIfFalse => 0x71,
            Opcode::JumpIfNull => 0x72,
            Opcode::Call => 0x73,
            Opcode::Return => 0x74,
            Opcode::Halt => 0x75,
            
            Opcode::MakeArray => 0x80,
            Opcode::MakeObject => 0x81,
            
            Opcode::LoopStart => 0x90,
            Opcode::LoopEnd => 0x91,
            
            Opcode::ForEach => 0xA0,
            Opcode::Filter => 0xA1,
            Opcode::Reduce => 0xA2,
            Opcode::Split => 0xA3,
            Opcode::Join => 0xA4,
            Opcode::Regex => 0xA5,
            Opcode::Hash => 0xA6,
            Opcode::Encrypt => 0xA7,
            Opcode::Decrypt => 0xA8,
            Opcode::TryStart => 0xA9,
            Opcode::TryEnd => 0xAA,
            Opcode::Retry => 0xAB,
            
            Opcode::Async => 0xB0,
            Opcode::Await => 0xB1,
            
            Opcode::DateTime => 0xC0,
            Opcode::Random => 0xC1,
            Opcode::Delta => 0xC2,
            
            Opcode::Import => 0xD0,
            Opcode::Log => 0xD1,
            Opcode::Debug => 0xD2,
            
            Opcode::TestStart => 0xE0,
            Opcode::Assert => 0xE1,
            Opcode::Mock => 0xE2,
            Opcode::BenchmarkStart => 0xE3,
            Opcode::BenchmarkEnd => 0xE4,
            
            Opcode::FileHandle => 0xF0,
            Opcode::FileRead => 0xF1,
            Opcode::FileWrite => 0xF2,
            Opcode::FileAppend => 0xF3,
            Opcode::Directory => 0xF4,
            Opcode::PathResolve => 0xF5,
            Opcode::DeleteFile => 0xF6,
            Opcode::SetPermission => 0xF7,
            
            // HTTP operations - using 0x5x range (after Input/Output)
            Opcode::HttpGet => 0x52,
            Opcode::HttpPost => 0x53,
            Opcode::HttpPut => 0x54,
            Opcode::HttpDelete => 0x55,
            Opcode::HttpPatch => 0x56,
            Opcode::HttpHead => 0x57,
            Opcode::HttpOptions => 0x58,
            
            // Process & Environment operations - using 0xDx range (after Debug)
            Opcode::EnvVar => 0xD3,
            Opcode::ProcessCreate => 0xD4,
            Opcode::ShellExec => 0xD5,
            Opcode::MemoryAlloc => 0xD6,
            Opcode::ExitProgram => 0xD7,
            Opcode::SendSignal => 0xD8,
            
            // Networking operations - using 0x6x range (after Query)
            Opcode::CreateSocket => 0x63,
            Opcode::ListenPort => 0x64,
            Opcode::ConnectRemote => 0x65,
            Opcode::PortNumber => 0x66,
            Opcode::CreatePacket => 0x67,
            Opcode::Handshake => 0x68,
            
            // Security operations - using 0xAx range (after Retry)
            Opcode::Sign => 0xAC,
            Opcode::VerifySignature => 0xAD,
            
            // Stream & Buffer operations - using 0x8x range (after MakeObject)
            Opcode::CreateStream => 0x82,
            Opcode::CreateBuffer => 0x83,
            Opcode::FlushBuffer => 0x84,
            Opcode::EndOfFile => 0x85,
            Opcode::SkipBytes => 0x86,
            
            // Concurrency operations - using 0xBx range (after Await)
            Opcode::Thread => 0xB2,
            Opcode::Lock => 0xB3,
            Opcode::Emit => 0xB4,
            Opcode::Watch => 0xB5,
            
            // Data operations - using 0x6x and 0x3x ranges
            Opcode::RegexMatch => 0x69,
            Opcode::Auth => 0x6A,
            Opcode::PropertyAccess => 0x35,
            
            // Math operations - using 0x2x range (after Root)
            Opcode::Infinity => 0x26,
            
            Opcode::End => 0xFF,
        }
    }
    
    /// Convert byte to opcode
    pub fn from_byte(byte: u8) -> Result<Self> {
        match byte {
            0x00 => Ok(Opcode::PushNull),
            0x01 => Ok(Opcode::PushBool),
            0x02 => Ok(Opcode::PushNumber),
            0x03 => Ok(Opcode::PushString),
            0x04 => Ok(Opcode::Pop),
            0x05 => Ok(Opcode::Dup),
            
            0x10 => Ok(Opcode::LoadVar),
            0x11 => Ok(Opcode::StoreVar),
            0x12 => Ok(Opcode::StoreImmutable),
            
            0x20 => Ok(Opcode::Add),
            0x21 => Ok(Opcode::Sub),
            0x22 => Ok(Opcode::Mul),
            0x23 => Ok(Opcode::Div),
            0x24 => Ok(Opcode::Power),
            0x25 => Ok(Opcode::Root),
            
            0x30 => Ok(Opcode::Equal),
            0x31 => Ok(Opcode::NotEqual),
            0x32 => Ok(Opcode::LessThan),
            0x33 => Ok(Opcode::GreaterThan),
            0x34 => Ok(Opcode::Approx),
            
            0x40 => Ok(Opcode::And),
            0x41 => Ok(Opcode::Or),
            0x42 => Ok(Opcode::Not),
            
            0x50 => Ok(Opcode::Input),
            0x51 => Ok(Opcode::Output),
            
            0x60 => Ok(Opcode::JsonParse),
            0x61 => Ok(Opcode::Persist),
            0x62 => Ok(Opcode::Query),
            
            0x70 => Ok(Opcode::Jump),
            0x71 => Ok(Opcode::JumpIfFalse),
            0x72 => Ok(Opcode::JumpIfNull),
            0x73 => Ok(Opcode::Call),
            0x74 => Ok(Opcode::Return),
            0x75 => Ok(Opcode::Halt),
            
            0x80 => Ok(Opcode::MakeArray),
            0x81 => Ok(Opcode::MakeObject),
            
            0x90 => Ok(Opcode::LoopStart),
            0x91 => Ok(Opcode::LoopEnd),
            
            0xA0 => Ok(Opcode::ForEach),
            0xA1 => Ok(Opcode::Filter),
            0xA2 => Ok(Opcode::Reduce),
            0xA3 => Ok(Opcode::Split),
            0xA4 => Ok(Opcode::Join),
            0xA5 => Ok(Opcode::Regex),
            0xA6 => Ok(Opcode::Hash),
            0xA7 => Ok(Opcode::Encrypt),
            0xA8 => Ok(Opcode::Decrypt),
            0xA9 => Ok(Opcode::TryStart),
            0xAA => Ok(Opcode::TryEnd),
            0xAB => Ok(Opcode::Retry),
            
            0xB0 => Ok(Opcode::Async),
            0xB1 => Ok(Opcode::Await),
            
            0xC0 => Ok(Opcode::DateTime),
            0xC1 => Ok(Opcode::Random),
            0xC2 => Ok(Opcode::Delta),
            
            0xD0 => Ok(Opcode::Import),
            0xD1 => Ok(Opcode::Log),
            0xD2 => Ok(Opcode::Debug),
            
            0xE0 => Ok(Opcode::TestStart),
            0xE1 => Ok(Opcode::Assert),
            0xE2 => Ok(Opcode::Mock),
            0xE3 => Ok(Opcode::BenchmarkStart),
            0xE4 => Ok(Opcode::BenchmarkEnd),
            
            0xF0 => Ok(Opcode::FileHandle),
            0xF1 => Ok(Opcode::FileRead),
            0xF2 => Ok(Opcode::FileWrite),
            0xF3 => Ok(Opcode::FileAppend),
            0xF4 => Ok(Opcode::Directory),
            0xF5 => Ok(Opcode::PathResolve),
            0xF6 => Ok(Opcode::DeleteFile),
            0xF7 => Ok(Opcode::SetPermission),
            
            // HTTP operations
            0x52 => Ok(Opcode::HttpGet),
            0x53 => Ok(Opcode::HttpPost),
            0x54 => Ok(Opcode::HttpPut),
            0x55 => Ok(Opcode::HttpDelete),
            0x56 => Ok(Opcode::HttpPatch),
            0x57 => Ok(Opcode::HttpHead),
            0x58 => Ok(Opcode::HttpOptions),
            
            // Process & Environment operations
            0xD3 => Ok(Opcode::EnvVar),
            0xD4 => Ok(Opcode::ProcessCreate),
            0xD5 => Ok(Opcode::ShellExec),
            0xD6 => Ok(Opcode::MemoryAlloc),
            0xD7 => Ok(Opcode::ExitProgram),
            0xD8 => Ok(Opcode::SendSignal),
            
            // Networking operations
            0x63 => Ok(Opcode::CreateSocket),
            0x64 => Ok(Opcode::ListenPort),
            0x65 => Ok(Opcode::ConnectRemote),
            0x66 => Ok(Opcode::PortNumber),
            0x67 => Ok(Opcode::CreatePacket),
            0x68 => Ok(Opcode::Handshake),
            
            // Security operations
            0xAC => Ok(Opcode::Sign),
            0xAD => Ok(Opcode::VerifySignature),
            
            // Stream & Buffer operations
            0x82 => Ok(Opcode::CreateStream),
            0x83 => Ok(Opcode::CreateBuffer),
            0x84 => Ok(Opcode::FlushBuffer),
            0x85 => Ok(Opcode::EndOfFile),
            0x86 => Ok(Opcode::SkipBytes),
            
            // Concurrency operations
            0xB2 => Ok(Opcode::Thread),
            0xB3 => Ok(Opcode::Lock),
            0xB4 => Ok(Opcode::Emit),
            0xB5 => Ok(Opcode::Watch),
            
            // Data operations
            0x69 => Ok(Opcode::RegexMatch),
            0x6A => Ok(Opcode::Auth),
            0x35 => Ok(Opcode::PropertyAccess),
            
            // Math operations
            0x26 => Ok(Opcode::Infinity),
            
            0xFF => Ok(Opcode::End),
            
            _ => Err(AetherError::BytecodeError(format!("Unknown opcode: 0x{:02X}", byte))),
        }
    }
}

/// Bytecode program structure
#[derive(Debug, Clone)]
pub struct BytecodeProgram {
    /// Constant pool (strings)
    pub constants: Vec<String>,
    /// Bytecode instructions
    pub code: Vec<u8>,
}

impl BytecodeProgram {
    /// Create a new empty bytecode program
    pub fn new() -> Self {
        BytecodeProgram {
            constants: Vec::new(),
            code: Vec::new(),
        }
    }
    
    /// Add a constant to the pool and return its index
    pub fn add_constant(&mut self, value: String) -> u32 {
        // Check if constant already exists
        if let Some(idx) = self.constants.iter().position(|s| s == &value) {
            return idx as u32;
        }
        
        let idx = self.constants.len() as u32;
        self.constants.push(value);
        idx
    }
    
    /// Emit an opcode
    pub fn emit_opcode(&mut self, opcode: Opcode) {
        self.code.push(opcode.to_byte());
    }
    
    /// Emit a u8 value
    pub fn emit_u8(&mut self, value: u8) {
        self.code.push(value);
    }
    
    /// Emit a u32 value (big-endian)
    pub fn emit_u32(&mut self, value: u32) {
        self.code.extend_from_slice(&value.to_be_bytes());
    }
    
    /// Emit a f64 value
    pub fn emit_f64(&mut self, value: f64) {
        self.code.extend_from_slice(&value.to_be_bytes());
    }
    
    /// Get current code position
    pub fn position(&self) -> usize {
        self.code.len()
    }
    
    /// Patch a u32 value at a specific position
    pub fn patch_u32(&mut self, position: usize, value: u32) {
        let bytes = value.to_be_bytes();
        self.code[position..position + 4].copy_from_slice(&bytes);
    }
    
    /// Serialize bytecode to writer
    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        // Write magic number
        writer.write_all(&MAGIC_NUMBER)
            .map_err(|e| AetherError::IoError(format!("Failed to write magic number: {}", e)))?;
        
        // Write version
        writer.write_all(&[VERSION])
            .map_err(|e| AetherError::IoError(format!("Failed to write version: {}", e)))?;
        
        // Write constant pool size
        let const_count = self.constants.len() as u32;
        writer.write_all(&const_count.to_be_bytes())
            .map_err(|e| AetherError::IoError(format!("Failed to write constant count: {}", e)))?;
        
        // Write constants
        for constant in &self.constants {
            let bytes = constant.as_bytes();
            let len = bytes.len() as u32;
            writer.write_all(&len.to_be_bytes())
                .map_err(|e| AetherError::IoError(format!("Failed to write constant length: {}", e)))?;
            writer.write_all(bytes)
                .map_err(|e| AetherError::IoError(format!("Failed to write constant: {}", e)))?;
        }
        
        // Write code size
        let code_size = self.code.len() as u32;
        writer.write_all(&code_size.to_be_bytes())
            .map_err(|e| AetherError::IoError(format!("Failed to write code size: {}", e)))?;
        
        // Write code
        writer.write_all(&self.code)
            .map_err(|e| AetherError::IoError(format!("Failed to write code: {}", e)))?;
        
        Ok(())
    }
    
    /// Deserialize bytecode from reader
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        // Read and verify magic number
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)
            .map_err(|e| AetherError::IoError(format!("Failed to read magic number: {}", e)))?;
        
        if magic != MAGIC_NUMBER {
            return Err(AetherError::BytecodeError("Invalid magic number".to_string()));
        }
        
        // Read and verify version
        let mut version = [0u8; 1];
        reader.read_exact(&mut version)
            .map_err(|e| AetherError::IoError(format!("Failed to read version: {}", e)))?;
        
        if version[0] != VERSION {
            return Err(AetherError::BytecodeError(format!(
                "Unsupported bytecode version: {}",
                version[0]
            )));
        }
        
        // Read constant pool size
        let mut const_count_bytes = [0u8; 4];
        reader.read_exact(&mut const_count_bytes)
            .map_err(|e| AetherError::IoError(format!("Failed to read constant count: {}", e)))?;
        let const_count = u32::from_be_bytes(const_count_bytes) as usize;
        
        // Read constants
        let mut constants = Vec::with_capacity(const_count);
        for _ in 0..const_count {
            let mut len_bytes = [0u8; 4];
            reader.read_exact(&mut len_bytes)
                .map_err(|e| AetherError::IoError(format!("Failed to read constant length: {}", e)))?;
            let len = u32::from_be_bytes(len_bytes) as usize;
            
            let mut bytes = vec![0u8; len];
            reader.read_exact(&mut bytes)
                .map_err(|e| AetherError::IoError(format!("Failed to read constant: {}", e)))?;
            
            let string = String::from_utf8(bytes)
                .map_err(|e| AetherError::BytecodeError(format!("Invalid UTF-8 in constant: {}", e)))?;
            constants.push(string);
        }
        
        // Read code size
        let mut code_size_bytes = [0u8; 4];
        reader.read_exact(&mut code_size_bytes)
            .map_err(|e| AetherError::IoError(format!("Failed to read code size: {}", e)))?;
        let code_size = u32::from_be_bytes(code_size_bytes) as usize;
        
        // Read code
        let mut code = vec![0u8; code_size];
        reader.read_exact(&mut code)
            .map_err(|e| AetherError::IoError(format!("Failed to read code: {}", e)))?;
        
        Ok(BytecodeProgram { constants, code })
    }
}

impl Default for BytecodeProgram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_opcode_roundtrip() {
        let opcodes = vec![
            Opcode::PushNull,
            Opcode::PushBool,
            Opcode::Add,
            Opcode::Output,
            Opcode::End,
        ];
        
        for opcode in opcodes {
            let byte = opcode.to_byte();
            let decoded = Opcode::from_byte(byte).unwrap();
            assert_eq!(opcode, decoded);
        }
    }
    
    #[test]
    fn test_bytecode_serialization() {
        let mut program = BytecodeProgram::new();
        
        // Add some constants
        let idx1 = program.add_constant("Hello".to_string());
        let idx2 = program.add_constant("World".to_string());
        
        // Emit some instructions
        program.emit_opcode(Opcode::PushString);
        program.emit_u32(idx1);
        program.emit_opcode(Opcode::PushString);
        program.emit_u32(idx2);
        program.emit_opcode(Opcode::Output);
        program.emit_opcode(Opcode::End);
        
        // Serialize
        let mut buffer = Vec::new();
        program.serialize(&mut buffer).unwrap();
        
        // Deserialize
        let mut cursor = Cursor::new(buffer);
        let deserialized = BytecodeProgram::deserialize(&mut cursor).unwrap();
        
        // Verify
        assert_eq!(program.constants, deserialized.constants);
        assert_eq!(program.code, deserialized.code);
    }
    
    #[test]
    fn test_constant_deduplication() {
        let mut program = BytecodeProgram::new();
        
        let idx1 = program.add_constant("test".to_string());
        let idx2 = program.add_constant("test".to_string());
        
        assert_eq!(idx1, idx2);
        assert_eq!(program.constants.len(), 1);
    }
}
