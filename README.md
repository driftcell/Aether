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

### Symbol Reference

| Symbol | Meaning | Description |
|--------|---------|-------------|
| `ƒ` | Function | Function definition |
| `λ` | Lambda | Anonymous function |
| `📥` | Input | Input/Request context |
| `📤` | Output | Output/Response |
| `💾` | Persist | Database/Storage operation |
| `🔍` | Query | Search/Query operation |
| `⇢` | Pipe | Data flow/pipe |
| `▷` | PipeInto | Bind to variable |
| `J` | JSON | Parse JSON |
| `⁇` | Guard | Null/validation check |
| `🛑` | Halt | Terminate with error |
| `✓` | Success | Validation success |
| `⨠` | Sequence | Sequential operations |
| `◇` | If | Conditional |
| `⊕` | Or | Logical OR |
| `⊗` | And | Logical AND |
| `¬` | Not | Logical NOT |
| `🗂` | Array | Array/List |
| `🗄` | Map | Map/Dictionary |
| `∅` | Empty | Null/Empty value |
| `🌐` | HTTP | HTTP request |
| `®` | Register | Create/Register |

## 🆕 What's New in v1.1

Aether v1.1 introduces **23 new symbols** across four major categories:

### Control Flow & Iteration
- `↻` Loop/While - Unbounded loops
- `∀` ForEach/Map - Collection iteration
- `∃` Filter/Find - Predicate-based filtering
- `∑` Reduce/Sum - Aggregation operations
- `🛡` Try/Rescue - Exception handling
- `♻` Retry - Failure retry mechanism

### Concurrency & Async
- `⚡` Async - Asynchronous execution
- `⏳` Await - Wait for async results
- `🧵` Thread - Concurrent task spawning
- `🔒` Lock - Mutex/critical sections
- `📡` Emit - Event broadcasting
- `👁` Watch - Event listening

### Data Manipulation
- `✂` Split - String/array splitting
- `🔗` Join - Element concatenation
- `✱` Regex - Pattern matching
- `≡` Equal - Strict equality
- `≠` NotEqual - Inequality comparison
- `🧊` Immutable - Constant definition

### System & Environment
- `🧩` Import - Module loading
- `🔑` Auth - Authentication/tokens
- `📅` DateTime - Time operations
- `🎲` Random - Random generation
- `🪵` Log - Logging output

**Example - Concurrent Web Crawler:**
```aether
🧩🌐 ⨠ 🗂urls ▷ ∀(u): (⚡ 🛡(♻3: 🌐📥u)) ▷ res ⁇ 🛑 ⨠ ∃(res.ok) ▷ 💾
```
*(70 characters vs 300+ in traditional JavaScript - 4.3x compression!)*

## 🆕 What's New in v1.2

Aether v1.2 introduces **15 new symbols** focused on testing, security, and scientific computing:

### Testing & Debugging
- `🧪` Test/Suite - Define test cases or test suites
- `⚖️` Assert - Assertion/verification (throws error if false)
- `🎭` Mock/Stub - Mock external dependencies
- `⏱️` Benchmark - Measure execution time
- `🐛` Debug - Debug mode/breakpoint

### Security & Crypto
- `🔐` Encrypt - Encrypt data with key
- `🔓` Decrypt - Decrypt data
- `#️⃣` Hash - Calculate hash value (SHA/MD5)
- `✍️` Sign - Digital signature
- `🛡️` Verify - Verify signature (distinct from 🛡 Try)

### Math & Science
- `↑` Power - Power operation (e.g., 2↑3 = 8)
- `√` Root - Square root
- `≈` Approx - Approximate equality
- `∞` Infinity - Infinity value
- `∆` Delta - Change/difference value

**Example - Secure Password Storage:**
```aether
ƒ®: 📥pass ▷ p ⨠ 🎲 ▷ salt ⨠ (p 🔗 salt) ⇢ #️⃣ ▷ hash ⨠ 💾{h:hash, s:salt} ⨠ 📤✓
```

**Example - Unit Testing:**
```aether
🧪 "AuthTest": 🎭💾 ⨠ ⏱️("admin" ⇢ ® ▷ res) ▷ time ⨠ ⚖️(res ≡ 200) ⨠ ⚖️(time < 50ms)
```

**Example - Vector Distance:**
```aether
ƒ calc: 📥v1 📥v2 ⨠ ((v1 - v2)↑2) ⇢ √ ▷ dist ⨠ ◇(dist ≈ 0) 📤"Same"
```

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

# Run an Aether program
aether run examples/hello.ae

# Display help
aether help
```

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

## 🎨 Projectional Editing (Future)

The vision includes IDE support with dual views:

- **Machine Layer**: High-density Aether symbols
- **Human Layer**: Real-time rendered readable code

```
Machine:  ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200
Human:    function register(input) {
            let user = input.parseJSON();
            if (!user) return error(400);
            database.save(user);
            return 200;
          }
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_lexer_basic_symbols
```

## 🛣️ Roadmap

- [x] Core language design
- [x] Lexer implementation
- [x] Parser with AST generation
- [x] Basic runtime execution
- [x] Symbol system
- [x] CLI interface
- [x] **v1.1: Extended symbol system** (Control Flow, Async, Data Manipulation, System)
- [x] **v1.2: Testing, Security & Math symbols** (Testing/Debugging, Security/Crypto, Math/Science)
- [ ] Full runtime with I/O
- [ ] Database connectors
- [ ] HTTP client/server
- [ ] Type system
- [ ] Compiler optimizations
- [ ] WASM target
- [ ] VSCode extension
- [ ] AI tokenizer integration
- [ ] Standard library

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
