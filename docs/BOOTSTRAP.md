# Aether Compiler Bootstrapping (自举)

## Overview

Compiler bootstrapping (自举) is the process of writing a compiler for a programming language using that same language. This document describes the approach used for Aether compiler self-hosting.

## Bootstrapping Stages

### Stage 0: Initial Compiler (Current)
The Aether compiler is currently implemented in Rust. This provides:
- Lexer (`src/lexer.rs`)
- Parser (`src/parser.rs`) 
- Runtime (`src/runtime.rs`)
- Virtual Machine (`src/vm.rs`)
- Bytecode compiler (`src/compiler.rs`)

### Stage 1: Bootstrap Operations (v1.4)
To enable self-hosting, we added the following bootstrap operations:

| Symbol | Operation | Description |
|--------|-----------|-------------|
| `📏` | Length | Get string/array length |
| `[ ]` | Index | Access array/string by index |
| `+` | Add | Arithmetic addition |
| `-` | Subtract | Arithmetic subtraction |
| `⧺` | Concat | String concatenation |
| `⊞` | Push | Push element to array |
| `[ ]` | Array Literal | Create array |
| `{ }` | Object Literal | Create object |

### Stage 2: Self-Hosting Lexer
With the bootstrap operations, we can write a lexer in Aether:

\`\`\`aether
// Simple tokenizer in Aether
"📤 42" ▷ source ⨠
source ⇢ 📏 ▷ len ⨠
[] ▷ tokens ⨠
0 ▷ pos ⨠

↻(pos < len): (
  source[pos] ▷ ch ⨠
  ◇(ch ≡ "📤"): (
    tokens ⊞ {type: "OUTPUT", value: ch} ▷ tokens ⨠
    pos + 1 ▷ pos
  )
  ◆: pos + 1 ▷ pos
) ⨠

📤 tokens
\`\`\`

### Stage 3: Self-Hosting Parser (Future)
Once the lexer is complete, the parser can be implemented in Aether to produce AST nodes.

### Stage 4: Full Bootstrap (Future)
The complete compiler including code generation can be written in Aether.

## Why Bootstrap?

1. **Language Expressiveness**: A language capable of implementing its own compiler demonstrates sufficient expressiveness.

2. **Dogfooding**: Using the language to write itself exposes deficiencies and drives improvements.

3. **Independence**: Eventually, the language can be compiled without external dependencies.

## Bootstrap Operations Examples

### Array Operations
\`\`\`aether
// Create and manipulate arrays
[1, 2, 3] ▷ arr ⨠
arr ⇢ 📏 ▷ len ⨠           // len = 3
arr[0] ▷ first ⨠           // first = 1
arr ⊞ 4 ▷ arr              // arr = [1, 2, 3, 4]
\`\`\`

### String Operations
\`\`\`aether
// String manipulation
"Hello" ▷ str ⨠
str ⇢ 📏 ▷ len ⨠           // len = 5
str[0] ▷ ch ⨠              // ch = "H"
str ⧺ " World" ▷ greeting  // greeting = "Hello World"
\`\`\`

### Arithmetic
\`\`\`aether
// Math operations for position tracking
5 + 3 ▷ sum ⨠              // sum = 8
10 - 4 ▷ diff ⨠            // diff = 6
\`\`\`

### Object Literals
\`\`\`aether
// Create token objects
{type: "OUTPUT", value: "📤", position: 0} ▷ token
\`\`\`

## Running Bootstrap Examples

\`\`\`bash
# Run bootstrap operation tests
./target/release/aether run examples/bootstrap_test.ae

# Run simple lexer demo
./target/release/aether run bootstrap/simple_lexer.ae
\`\`\`

## Current Status

- ✅ Bootstrap operations implemented (v1.4)
- ✅ All 132 tests passing
- ✅ Basic lexer demo working
- 🚧 Full self-hosting lexer (in progress)
- 📋 Self-hosting parser (planned)
- 📋 Full bootstrap (planned)

## References

- [Bootstrapping (compilers)](https://en.wikipedia.org/wiki/Bootstrapping_(compilers))
- Aether Symbol Reference: `docs/SYMBOL_REFERENCE.md`
