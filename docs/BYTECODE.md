# Aether Bytecode Format (.aeb)

## Overview

Aether bytecode (.aeb files) is a compact binary format that represents compiled Aether programs. The bytecode is designed to be portable, efficient, and easy to execute on a stack-based virtual machine.

## File Format

### Header Structure

```
[Magic Number] [Version] [Constant Pool] [Code Section]
```

#### Magic Number (4 bytes)
- Bytes: `41 45 42 00` (ASCII: "AEB\0")
- Used to identify valid Aether bytecode files

#### Version (1 byte)
- Current version: `01`
- Allows for future format changes

#### Constant Pool
- **Size** (4 bytes, big-endian): Number of constants
- **Constants**: Variable-length strings
  - Each constant: `[Length (4 bytes)][UTF-8 bytes]`
  
#### Code Section
- **Size** (4 bytes, big-endian): Number of code bytes
- **Code**: Sequence of bytecode instructions

## Example: Hello World

Source (`hello.ae`):
```aether
📤 "Hello, Aether!"
```

Bytecode hex dump (`hello.aeb`):
```
00000000  41 45 42 00 01 00 00 00  01 00 00 00 0e 48 65 6c  |AEB..........Hel|
00000010  6c 6f 2c 20 41 65 74 68  65 72 21 00 00 00 07 03  |lo, Aether!.....|
00000020  00 00 00 00 51 ff                                 |....Q.|
```

Breakdown:
- `41 45 42 00` - Magic number "AEB\0"
- `01` - Version 1
- `00 00 00 01` - 1 constant in pool
- `00 00 00 0e` - Constant length: 14 bytes
- `48 65 6c 6c 6f 2c 20 41 65 74 68 65 72 21` - "Hello, Aether!"
- `00 00 00 07` - Code size: 7 bytes
- `03 00 00 00 00` - PushString instruction (0x03) with index 0
- `51` - Output instruction (0x51)
- `ff` - End instruction (0xFF)

## Instruction Set

### Stack Operations (0x00-0x0F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x00 | PushNull | Push null onto stack |
| 0x01 | PushBool | Push boolean (next byte: 0=false, 1=true) |
| 0x02 | PushNumber | Push f64 number (next 8 bytes) |
| 0x03 | PushString | Push string from constant pool (next 4 bytes: index) |
| 0x04 | Pop | Remove top value from stack |
| 0x05 | Dup | Duplicate top value |

### Variable Operations (0x10-0x1F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x10 | LoadVar | Load variable (4 bytes: name index) |
| 0x11 | StoreVar | Store variable (4 bytes: name index) |
| 0x12 | StoreImmutable | Store immutable variable (4 bytes: name index) |

### Arithmetic Operations (0x20-0x2F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x20 | Add | Add top two numbers |
| 0x21 | Sub | Subtract top two numbers |
| 0x22 | Mul | Multiply top two numbers |
| 0x23 | Div | Divide top two numbers |
| 0x24 | Power | Raise to power (base^exponent) |
| 0x25 | Root | Square root |

### Comparison Operations (0x30-0x3F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x30 | Equal | Check equality |
| 0x31 | NotEqual | Check inequality |
| 0x32 | LessThan | Compare less than |
| 0x33 | GreaterThan | Compare greater than |
| 0x34 | Approx | Approximate equality (epsilon: 0.000001) |

### Logical Operations (0x40-0x4F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x40 | And | Logical AND |
| 0x41 | Or | Logical OR |
| 0x42 | Not | Logical NOT |

### I/O Operations (0x50-0x5F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x50 | Input | Read input |
| 0x51 | Output | Output value |

### Data Operations (0x60-0x6F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x60 | JsonParse | Parse JSON |
| 0x61 | Persist | Persist to storage |
| 0x62 | Query | Query operation |

### Control Flow (0x70-0x7F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x70 | Jump | Unconditional jump (4 bytes: target) |
| 0x71 | JumpIfFalse | Jump if false (4 bytes: target) |
| 0x72 | JumpIfNull | Jump if null (4 bytes: target) |
| 0x73 | Call | Call function (4 bytes: func index, 1 byte: arg count) |
| 0x74 | Return | Return from function |
| 0x75 | Halt | Halt with error |

### Collections (0x80-0x8F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x80 | MakeArray | Create array (4 bytes: element count) |
| 0x81 | MakeObject | Create object (4 bytes: pair count) |

### Loops (0x90-0x9F)

| Opcode | Name | Description |
|--------|------|-------------|
| 0x90 | LoopStart | Start loop (4 bytes: end offset) |
| 0x91 | LoopEnd | End loop (jump to start) |

### Advanced Operations (0xA0-0xAF)

| Opcode | Name | Description |
|--------|------|-------------|
| 0xA0 | ForEach | Iterate collection (4 bytes: var name) |
| 0xA1 | Filter | Filter operation |
| 0xA2 | Reduce | Reduce/fold operation |
| 0xA3 | Split | Split string/array |
| 0xA4 | Join | Join elements |
| 0xA5 | Regex | Regex match |
| 0xA6 | Hash | Hash calculation |
| 0xA7 | Encrypt | Encrypt data |
| 0xA8 | Decrypt | Decrypt data |
| 0xA9 | TryStart | Try-rescue start (4 bytes: rescue offset) |
| 0xAA | TryEnd | Try-rescue end |
| 0xAB | Retry | Retry operation (1 byte: max attempts) |

### Async Operations (0xB0-0xBF)

| Opcode | Name | Description |
|--------|------|-------------|
| 0xB0 | Async | Async operation start |
| 0xB1 | Await | Await async result |

### Time & Random (0xC0-0xCF)

| Opcode | Name | Description |
|--------|------|-------------|
| 0xC0 | DateTime | Get current datetime |
| 0xC1 | Random | Generate random value |
| 0xC2 | Delta | Calculate delta/difference |

### System Operations (0xD0-0xDF)

| Opcode | Name | Description |
|--------|------|-------------|
| 0xD0 | Import | Import module (4 bytes: name index) |
| 0xD1 | Log | Log message |
| 0xD2 | Debug | Debug breakpoint |

### Testing Operations (0xE0-0xEF)

| Opcode | Name | Description |
|--------|------|-------------|
| 0xE0 | TestStart | Start test (4 bytes: name index) |
| 0xE1 | Assert | Assert condition |
| 0xE2 | Mock | Mock operation (4 bytes: target index) |
| 0xE3 | BenchmarkStart | Start benchmark |
| 0xE4 | BenchmarkEnd | End benchmark |

### File Operations (0xF0-0xFE)

| Opcode | Name | Description |
|--------|------|-------------|
| 0xF0 | FileHandle | Get file handle |
| 0xF1 | FileRead | Read from file |
| 0xF2 | FileWrite | Write to file |
| 0xF3 | FileAppend | Append to file |

### Special (0xFF)

| Opcode | Name | Description |
|--------|------|-------------|
| 0xFF | End | End of program |

## Virtual Machine

The Aether VM is a stack-based virtual machine with the following components:

### Architecture

1. **Program Counter (PC)**: Points to current instruction
2. **Value Stack**: Holds runtime values
3. **Variables**: HashMap for variable storage
4. **Immutable Variables**: HashSet for constant tracking
5. **Call Stack**: Function return addresses

### Execution Model

1. Load bytecode program
2. Initialize PC to 0
3. Loop until End instruction or PC exceeds code length:
   - Read opcode at PC
   - Execute instruction
   - Update PC
4. Return top of stack as result

### Safety Features

- Maximum iteration limit (10,000 by default)
- Stack underflow protection
- Immutable variable protection
- Division by zero checks
- Bytecode bounds checking

## Usage

### Compile Source to Bytecode

```bash
# Compile .ae to .aeb
aether compile program.ae

# Specify output file
aether compile program.ae output.aeb
```

### Execute Bytecode

```bash
# Execute .aeb file
aether exec program.aeb
```

### Complete Workflow

```bash
# Write Aether source
echo '📤 "Hello, World!"' > hello.ae

# Compile to bytecode
aether compile hello.ae

# Execute bytecode
aether exec hello.aeb
```

## Benefits of Bytecode

1. **Portability**: Bytecode can be executed on any platform with an Aether VM
2. **Performance**: No parsing overhead during execution
3. **Distribution**: Ship compiled bytecode instead of source
4. **Security**: Obfuscates source code
5. **Size**: Compact binary representation
6. **Validation**: Pre-validated at compile time

## Future Enhancements

- **JIT Compilation**: Compile hot paths to native code
- **Optimization**: Constant folding, dead code elimination
- **AOT Compilation**: Compile to native executables
- **Bytecode Verification**: Additional safety checks
- **Debugger Support**: Breakpoints and step execution
- **Profiling**: Performance analysis tools

## Technical Notes

### Endianness

All multi-byte values use **big-endian** encoding for portability across architectures.

### String Encoding

All strings in the constant pool use **UTF-8** encoding, consistent with Aether's Unicode-first design.

### Version Compatibility

The VM checks bytecode version on load. Future versions may:
- Add new opcodes (maintaining backward compatibility)
- Change instruction formats (version increment required)
- Optimize encoding (with migration tools)

### Error Handling

The VM propagates errors with context:
- Stack underflow → RuntimeError
- Invalid opcode → BytecodeError
- Division by zero → RuntimeError
- Type mismatch → RuntimeError

## Example Programs

### Arithmetic

Source:
```aether
📤 (10 + 5)
```

Instructions:
```
PushNumber 10.0
PushNumber 5.0
Add
Output
End
```

### Variables

Source:
```aether
📥 ▷ x ⨠ 📤 x
```

Instructions:
```
Input
Dup
StoreVar "x"
LoadVar "x"
Output
End
```

### Conditional

Source:
```aether
◇(x > 10): 📤 "Large"
```

Instructions:
```
LoadVar "x"
PushNumber 10.0
GreaterThan
JumpIfFalse [end]
PushString "Large"
Output
[end]
End
```

---

**Note**: This bytecode format is designed for Aether v0.1.0 and may evolve with future language versions.
