//! Error types for the Aether language

use thiserror::Error;

/// Result type for Aether operations
pub type Result<T> = std::result::Result<T, AetherError>;

/// Errors that can occur during Aether compilation and execution
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AetherError {
    #[error("Lexer error: {0}")]
    LexerError(String),

    #[error("Parser error: {0}")]
    ParserError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Bytecode error: {0}")]
    BytecodeError(String),

    #[error("Compiler error: {0}")]
    CompilerError(String),
}

impl From<std::io::Error> for AetherError {
    fn from(err: std::io::Error) -> Self {
        AetherError::IoError(err.to_string())
    }
}
