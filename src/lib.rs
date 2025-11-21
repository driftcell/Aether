//! # Aether (以太) - The First AI-Native Programming Language
//!
//! Aether is a revolutionary programming language designed for AI-first code generation.
//! Instead of ASCII-constrained keywords, it uses high-density UTF-8 symbols to maximize
//! information density and minimize token usage for AI models.
//!
//! ## Core Concepts
//!
//! - **Symbol-based syntax**: Uses UTF-8 characters for maximum information density
//! - **AI-optimized**: Designed to minimize AI token consumption
//! - **Projectional editing**: Human-readable projection over machine-optimized code
//! - **High-performance runtime**: Compiles to efficient native code
//!
//! ## Example
//!
//! ```text
//! ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200
//! ```
//!
//! This represents a user registration endpoint that:
//! - Parses JSON input
//! - Validates user data
//! - Persists to database
//! - Returns success response

pub mod error;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod symbols;
pub mod bytecode;
pub mod compiler;
pub mod vm;

pub use error::{AetherError, Result};
pub use lexer::{Lexer, Token, TokenType};
pub use parser::{AstNode, Parser};
pub use runtime::Runtime;
pub use symbols::Symbol;
pub use bytecode::{BytecodeProgram, Opcode};
pub use compiler::Compiler;
pub use vm::VM;

/// Version information for the Aether language
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Language name
pub const LANGUAGE_NAME: &str = "Aether (以太)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_language_name() {
        assert_eq!(LANGUAGE_NAME, "Aether (以太)");
    }
}
