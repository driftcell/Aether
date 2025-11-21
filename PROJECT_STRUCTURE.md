# Aether Project Structure

## Root Directory

```
Aether/
├── docs/           # All documentation files
├── examples/       # Example Aether programs (.ae files)
├── scripts/        # Build and test scripts
├── src/            # Rust source code
├── CHANGELOG.md    # Version history and release notes
├── README.md       # Main project documentation
├── Cargo.toml      # Rust project configuration
└── LICENSE-*       # License files
```

## Documentation (`docs/`)

- `SYMBOL_REFERENCE.md` - Complete symbol listing and descriptions
- `ARCHITECTURE.md` - System architecture and design
- `BYTECODE.md` - Bytecode format specification
- `PROJECT_SUMMARY.md` - High-level project overview
- `BYTECODE_DEMO.txt` - Bytecode examples and demos
- `CONDITIONALS_SUMMARY.md` - Conditional logic documentation
- `IMPLEMENTATION_SUMMARY.md` - Implementation details
- `PR_SUMMARY.md` - Pull request summaries
- `TEST_COVERAGE.md` - Test coverage information

## Examples (`examples/`)

Contains `.ae` source files demonstrating various language features:
- Basic operations (hello, number, arithmetic, variable)
- Control flow (conditional, guard, if_else)
- Math operations (power, sqrt, complex_math, delta)
- Data manipulation (split, join, string_concat)
- I/O operations (file_read, log)
- HTTP operations (http_simple, http_requests, api_demo)
- Advanced features (async, foreach, retry, immutable)

## Scripts (`scripts/`)

- `test_examples.sh` - Comprehensive test suite for compiling and executing all examples

## Source Code (`src/`)

- `main.rs` - CLI entry point
- `lib.rs` - Library interface
- `lexer.rs` - Tokenization
- `parser.rs` - AST generation
- `compiler.rs` - Bytecode compilation
- `runtime.rs` - AST execution
- `vm.rs` - Bytecode VM
- `symbols.rs` - Symbol definitions
- `bytecode.rs` - Bytecode format
- `constants.rs` - Shared constants
- `error.rs` - Error types

## Quick Links

- **Getting Started**: See [README.md](README.md#-getting-started)
- **Symbol Reference**: See [docs/SYMBOL_REFERENCE.md](docs/SYMBOL_REFERENCE.md)
- **Version History**: See [CHANGELOG.md](CHANGELOG.md)
- **Run Tests**: `./scripts/test_examples.sh`
