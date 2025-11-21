# Aether (以太) - The First AI-Native Programming Language

[![Rust](https://img.shields.io/badge/rust-1.91+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

> **Breaking free from ASCII's gravity: A programming language designed for AI, not keyboards.**

## 🌟 Vision

In the era of AI code generation, why are we still constrained by ASCII characters designed for 1960s teletype machines? Aether challenges this paradigm by introducing a **high-density, UTF-8 symbol-based programming language** optimized for AI token efficiency and computational thinking.

### The Problem with Traditional Languages

- **ASCII Limitations**: Confined to 128 characters, forcing verbose keywords like `function`, `return`, `import`
- **Human-Centric**: Designed for keyboard typing and human readability at the cost of information density
- **Token Inefficiency**: For AI models, `function` consumes multiple tokens, while `ƒ` uses only one

### The Aether Solution

Aether leverages the full **UTF-8 character space** to create a "modern digital hieroglyphic" system where:
- **1 symbol = 1 concept** (maximum information density)
- **AI-optimized** token consumption
- **Human collaboration** through projectional editing
- **Native performance** via Rust compilation

## 🎯 Core Concepts

### Symbol-Based Syntax

Traditional Python (~100 characters):
```python
def register(ctx):
    user = ctx.json()
    if not user.email: return error(400)
    db.save(user)
    return 200
```

Aether GlyphCode (~20 characters):
```aether
ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200
```

### Core Symbol Reference

Common symbols for everyday use:

| Symbol | Meaning | Description |
|--------|---------|-------------|
| `ƒ` | Function | Function definition |
| `📥` | Input | Input/Request context |
| `📤` | Output | Output/Response |
| `⇢` | Pipe | Data flow/pipe |
| `▷` | PipeInto | Bind to variable |
| `⨠` | Sequence | Sequential operations |
| `⁇` | Guard | Null/validation check |
| `🛑` | Halt | Terminate with error |
| `◇` | If | Conditional (if) |
| `💾` | Persist | Database/Storage operation |

📚 For a complete symbol reference, see [docs/SYMBOL_REFERENCE.md](docs/SYMBOL_REFERENCE.md)

## ✨ Features

- **Symbol-Based Syntax**: One symbol per concept for maximum information density
- **AI-Optimized**: Designed for efficient token consumption in AI models
- **UTF-8 Native**: Leverages the full Unicode character space
- **Type Safe**: Built on Rust's strong type system
- **Native Performance**: Compiles to efficient native code
- **Bytecode Support**: Compile to .aeb bytecode for faster execution and distribution
- **Async Runtime**: Full async/await support with tokio for concurrent execution

**Latest version**: v1.6 with async runtime using tokio

📋 See [CHANGELOG.md](CHANGELOG.md) for version history and detailed feature updates.
📘 See [docs/ASYNC_RUNTIME.md](docs/ASYNC_RUNTIME.md) for async/await documentation.


## 🚀 Getting Started

### Prerequisites

- Rust 1.91 or higher
- UTF-8 capable terminal

### Installation

```bash
# Clone the repository
git clone https://github.com/driftcell/Aether.git
cd Aether

# Build the project
cargo build --release

# Run the CLI
./target/release/aether --help
```

### Usage

```bash
# Display version
aether version

# View symbol reference
aether symbols

# Run an Aether program directly
aether run examples/hello.ae

# Compile to bytecode
aether compile examples/hello.ae

# Execute bytecode
aether exec examples/hello.aeb

# Explain Aether code in human-readable format
aether explain examples/register.ae

# Explain multiple files using glob patterns
aether explain examples/*.ae

# Display help
aether help
```

### Projectional Editing with `explain`

Aether's dense symbol-based syntax can be challenging for humans to read. The `explain` command provides instant translation to human-readable pseudo-code:

```bash
# Explain a single file
aether explain examples/register.ae

# Explain multiple files
aether explain examples/async*.ae

# Explain all examples
aether explain examples/*.ae
```

**Example transformation:**
```aether
Input (Aether):  ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200

Output (Human):  function register:
                   read input | parse JSON(read input) -> store in u
                 guard (if u is null or invalid):
                   halt with error 400
                 then
                 save to database: u
                 then
                 output 200
```

### Bytecode Compilation Workflow

Aether now supports compilation to bytecode (.aeb files) for improved performance and distribution:

```bash
# Write your Aether code
echo '📤 "Hello, World!"' > program.ae

# Compile to bytecode
aether compile program.ae

# Execute the bytecode
aether exec program.aeb
```

**Benefits of Bytecode:**
- ⚡ Faster execution (no parsing overhead)
- 📦 Compact binary format with constant pooling
- 🔒 Obfuscated source code
- 🚀 Distribute compiled programs
- ✅ Pre-validated at compile time

See [docs/BYTECODE.md](docs/BYTECODE.md) for detailed bytecode format specification.

## 📚 Examples

### Hello World
```aether
📤 "Hello, Aether!"
```

### User Registration Endpoint
```aether
ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200
```

**Explanation:**
- `ƒ®:` - Define function named "register"
- `📥⇢J` - Take input and pipe to JSON parser
- `▷ u` - Bind result to variable `u`
- `⁇` - Guard: check if value is null/invalid
- `🛑400` - If invalid, halt with error code 400
- `⨠` - Then (sequence operator)
- `💾u` - Persist variable `u` to database
- `⨠` - Then
- `📤200` - Output success code 200

### Data Processing Pipeline
```aether
📥 ⇢J ▷ data ⨠ 🔍data ⨠ 📤
```

**Explanation:**
- Read input, parse JSON, bind to `data`
- Query/filter the data
- Output the result

## 🏗️ Architecture

### Components

1. **Lexer** (`src/lexer.rs`)
   - Tokenizes UTF-8 source code
   - Handles emoji and Unicode symbols
   - Produces token stream for parser

2. **Parser** (`src/parser.rs`)
   - Builds Abstract Syntax Tree (AST)
   - Validates syntax structure
   - Optimizes symbol sequences

3. **Runtime** (`src/runtime.rs`)
   - Executes AST nodes
   - Manages variable scope
   - Handles I/O operations

4. **Symbol System** (`src/symbols.rs`)
   - Defines all language symbols
   - Provides bidirectional mapping
   - Documents symbol semantics

### Design Principles

- **Maximum Density**: One symbol per concept
- **AI-First**: Optimized for token efficiency
- **Type Safety**: Leverages Rust's type system
- **Performance**: Compiles to native code
- **Extensibility**: Easy to add new symbols

## 🎨 Projectional Editing

Aether now supports projectional editing through the `explain` command, which provides instant translation of dense symbol-based code into human-readable format.

**Current implementation:**
- **Machine Layer**: High-density Aether symbols (stored in .ae files)
- **Human Layer**: Real-time rendered readable code (via `explain` command)
- **CLI tool**: Use `aether explain <files>` to translate any .ae file

```
Machine:  ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200
Human:    function register:
            read input | parse JSON(read input) -> store in u
          guard (if u is null or invalid):
            halt with error 400
          then
          save to database: u
          then
          output 200
```

**Future enhancements:**
- IDE integration with dual-pane editing
- Bidirectional synchronization between layers
- Customizable human-readable formats
- Real-time inline translation

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_lexer_basic_symbols

# Test all examples (compile + execute)
./scripts/test_examples.sh
```

## 🛣️ Roadmap

### Completed ✅
- Core language design, lexer, parser, and runtime
- Symbol system with 100+ symbols across multiple domains
- CLI interface with bytecode compilation
- HTTP client with full REST API support
- **Async/await runtime with tokio for concurrent execution**

### In Progress 🚧
- Full runtime with I/O operations
- Database connectors
- HTTP server framework
- Type system

### Future Plans 🔮
- Compiler optimizations (constant folding, dead code elimination)
- Enhanced async features (cancellation, channels, async iterators)
- JIT compilation
- WASM target
- VSCode extension
- AI tokenizer integration
- Standard library

## 📚 Documentation

- [Symbol Reference](docs/SYMBOL_REFERENCE.md) - Complete list of all symbols
- [Async Runtime](docs/ASYNC_RUNTIME.md) - Async/await usage guide
- [Architecture](docs/ARCHITECTURE.md) - System architecture overview
- [Bytecode Format](docs/BYTECODE.md) - Bytecode specification
- [Project Summary](docs/PROJECT_SUMMARY.md) - High-level project overview
- [CHANGELOG](CHANGELOG.md) - Version history and updates

## 🤝 Contributing

Contributions are welcome! This is an experimental language pushing the boundaries of programming paradigms.

### Areas for Contribution

- Symbol design and semantics
- Runtime optimizations
- Documentation and examples
- IDE tooling
- AI model fine-tuning for Aether tokens

## 📖 Philosophy

> "In the AI era, code is **intent**, and symbols are **compute**."

Aether represents a paradigm shift:
- From **linear text** to **dense topology**
- From **human-optimized** to **AI-optimized**
- From **ASCII constraints** to **Unicode freedom**
- From **verbose keywords** to **symbolic logic**

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

Inspired by the vision of AI-native programming and the limitations of legacy text-based code.

## 📬 Contact

- GitHub: [driftcell/Aether](https://github.com/driftcell/Aether)
- Issues: [GitHub Issues](https://github.com/driftcell/Aether/issues)

---

**Aether (以太)** - Where symbols meet computation, and AI meets efficiency.
