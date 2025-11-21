//! Shared constants used across the Aether compiler and parser

/// Special variable name used for piped values in AST
/// This is used by both parser and compiler to represent the value passed through pipe operations (⇢)
pub const PIPE_VARIABLE: &str = "_pipe";
