# Aether Examples

This directory contains example Aether programs demonstrating various language features and bytecode compilation.

## Basic Examples

### hello.ae
```aether
📤 "Hello, Aether!"
```
Classic hello world program. Demonstrates string output.

### number.ae
```aether
📤 42
```
Outputs a numeric value. Shows number literals.

### sequence.ae
```aether
📤 "Starting..." ⨠ 📤 42 ⨠ 📤 "Done!"
```
Multiple sequential operations using the sequence operator (⨠).

## Advanced Examples

### datetime.ae
```aether
📤 📅
```
Outputs current date and time. Uses the DateTime symbol (📅).

### random.ae
```aether
📤 🎲
```
Generates and outputs a random number. Uses the Random symbol (🎲).

### http_requests.ae (Requires Network)
Demonstrates all HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS).
**Note**: Requires internet connectivity to httpbin.org

### api_demo.ae (Requires Network)
Practical API usage example with JSONPlaceholder API.
**Note**: Requires internet connectivity to jsonplaceholder.typicode.com

### http_simple.ae (Requires Network)
Simple HTTP GET request demonstration.
**Note**: Requires internet connectivity

## Compiling and Running Examples

### Option 1: Direct Interpretation (Run)
```bash
aether run examples/hello.ae
```

### Option 2: Compile to Bytecode + Execute
```bash
# Step 1: Compile .ae to .aeb
aether compile examples/hello.ae

# Step 2: Execute bytecode
aether exec examples/hello.aeb
```

### Option 3: Custom Output Path
```bash
aether compile examples/hello.ae output/hello.aeb
aether exec output/hello.aeb
```

## Running All Examples

Use the provided test script:
```bash
./test_examples.sh
```

This will:
1. Compile all examples to bytecode
2. Execute each bytecode file
3. Verify outputs
4. Display a summary

## Bytecode Files (.aeb)

After compilation, you'll find `.aeb` files alongside the source files. These are binary bytecode files that:
- Start with magic number "AEB\0"
- Contain a constant pool for strings
- Store optimized instructions
- Can be distributed without source code
- Execute faster than parsing source

Example bytecode sizes:
- `hello.aeb`: 38 bytes
- `number.aeb`: 24 bytes  
- `sequence.aeb`: 60 bytes
- `datetime.aeb`: 16 bytes
- `random.aeb`: 16 bytes

## Symbol Reference

Quick reference for symbols used in examples:

| Symbol | Name | Description |
|--------|------|-------------|
| 📤 | Output | Output a value |
| 📥 | Input | Read input |
| ⨠ | Sequence | Sequential operations |
| ▷ | PipeInto | Bind to variable |
| 📅 | DateTime | Get current time |
| 🎲 | Random | Random number |
| 💾 | Persist | Save to database |
| 🔍 | Query | Query data |
| ⇢ | Pipe | Data flow |
| J | JSON | Parse JSON |
| ⁇ | Guard | Null check |
| 🛑 | Halt | Error halt |
| 🌐📥 | HTTP GET | HTTP GET request |
| 🌐📤 | HTTP POST | HTTP POST request |
| 🌐🔄 | HTTP PUT | HTTP PUT request |
| 🌐🗑️ | HTTP DELETE | HTTP DELETE request |
| 🌐🔧 | HTTP PATCH | HTTP PATCH request |
| 🌐👁️ | HTTP HEAD | HTTP HEAD request |
| 🌐⚙️ | HTTP OPTIONS | HTTP OPTIONS request |

For complete symbol reference: `aether symbols`

## Creating Your Own Examples

1. Create a new `.ae` file in this directory
2. Write your Aether code using UTF-8 symbols
3. Compile: `aether compile examples/myfile.ae`
4. Execute: `aether exec examples/myfile.aeb`

## Performance Comparison

| Example | Source Size | Bytecode Size | Compression |
|---------|-------------|---------------|-------------|
| hello.ae | 19 bytes | 38 bytes | 2.0x larger* |
| number.ae | 4 bytes | 24 bytes | 6.0x larger* |
| sequence.ae | 48 bytes | 60 bytes | 1.3x larger* |
| datetime.ae | 4 bytes | 16 bytes | 4.0x larger* |

\* Bytecode includes headers and metadata. For larger programs, bytecode becomes more efficient.

## Troubleshooting

### Compilation Errors

**Error**: `Lexer error: Unknown character`
- Check that you're using valid Aether symbols
- Ensure your file is UTF-8 encoded
- Run `aether symbols` to see valid symbols

**Error**: `Parser error`
- Verify syntax is correct
- Check for matching parentheses/brackets
- Ensure sequence operators (⨠) are properly placed

### Execution Errors

**Error**: `Invalid magic number`
- File is not valid Aether bytecode
- File may be corrupted
- Recompile from source

**Error**: `Stack underflow`
- Bytecode is malformed
- Recompile from source
- Report as bug if persists

## Next Steps

1. Try modifying existing examples
2. Create new examples combining multiple features
3. Check `docs/BYTECODE.md` for bytecode format details
4. See `docs/SYMBOL_REFERENCE.md` for all available symbols
5. Read `docs/ARCHITECTURE.md` for implementation details
