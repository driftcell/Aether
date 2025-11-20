# Aether Symbol Reference Guide

Complete reference for all symbols in the Aether programming language.

## 📑 Table of Contents

1. [Function & Control Flow](#function--control-flow)
2. [Data Operations](#data-operations)
3. [Logic & Conditionals](#logic--conditionals)
4. [Collections](#collections)
5. [Literals & Types](#literals--types)
6. [Network Operations](#network-operations)

## Function & Control Flow

### ƒ - Function Definition

**Symbol**: `ƒ` (Latin Small Letter F with Hook)  
**Unicode**: U+0192  
**Purpose**: Define a named or anonymous function

**Syntax**:
```aether
ƒname: body
```

**Example**:
```aether
ƒ®: 📥⇢J ▷ u
```
Defines a function named "register" (using ® symbol)

---

### λ - Lambda Function

**Symbol**: `λ` (Greek Small Letter Lambda)  
**Unicode**: U+03BB  
**Purpose**: Create anonymous function

**Syntax**:
```aether
λ: expression
```

**Example**:
```aether
λ: x ⨠ 📤x
```
Anonymous function that returns its input

---

### ⇢ - Pipe

**Symbol**: `⇢` (Rightwards Two-Headed Arrow)  
**Unicode**: U+21E2  
**Purpose**: Pipe data from source to operation

**Syntax**:
```aether
source ⇢ operation
```

**Example**:
```aether
📥 ⇢ J
```
Pipe input through JSON parser

---

### ▷ - Pipe Into Variable

**Symbol**: `▷` (White Right-Pointing Triangle)  
**Unicode**: U+25B7  
**Purpose**: Bind piped value to variable

**Syntax**:
```aether
value ▷ varname
```

**Example**:
```aether
📥⇢J ▷ user
```
Parse JSON and bind to `user` variable

---

### ⨠ - Sequence

**Symbol**: `⨠` (Triple Vertical Bar Right Turnstile)  
**Unicode**: U+2A20  
**Purpose**: Chain multiple operations sequentially

**Syntax**:
```aether
op1 ⨠ op2 ⨠ op3
```

**Example**:
```aether
📥 ⨠ 💾 ⨠ 📤200
```
Input, persist, then output 200

---

## Data Operations

### 📥 - Input

**Symbol**: `📥` (Inbox Tray Emoji)  
**Unicode**: U+1F4E5  
**Purpose**: Read input/request context

**Syntax**:
```aether
📥
```

**Example**:
```aether
📥 ⇢ J
```
Read and parse JSON input

---

### 📤 - Output

**Symbol**: `📤` (Outbox Tray Emoji)  
**Unicode**: U+1F4E4  
**Purpose**: Output/return value

**Syntax**:
```aether
📤 value
```

**Example**:
```aether
📤 200
📤 "success"
```

---

### 💾 - Persist

**Symbol**: `💾` (Floppy Disk Emoji)  
**Unicode**: U+1F4BE  
**Purpose**: Save to database/storage

**Syntax**:
```aether
💾 value
```

**Example**:
```aether
💾 user
```
Persist user data to database

---

### 🔍 - Query

**Symbol**: `🔍` (Magnifying Glass Emoji)  
**Unicode**: U+1F50D  
**Purpose**: Search/query data

**Syntax**:
```aether
🔍 criteria
```

**Example**:
```aether
🔍 users
```
Query users from database

---

### J - JSON Parse

**Symbol**: `J`  
**Unicode**: U+004A  
**Purpose**: Parse JSON data

**Syntax**:
```aether
source ⇢ J
```

**Example**:
```aether
📥 ⇢ J ▷ data
```
Parse input JSON into data variable

---

## Logic & Conditionals

### ⁇ - Guard

**Symbol**: `⁇` (Double Question Mark)  
**Unicode**: U+2047  
**Purpose**: Check for null/invalid and branch

**Syntax**:
```aether
value ⁇ alternative
```

**Example**:
```aether
user ⁇ 🛑400
```
If user is null, halt with 400 error

---

### 🛑 - Halt

**Symbol**: `🛑` (Stop Sign Emoji)  
**Unicode**: U+1F6D1  
**Purpose**: Terminate with error code

**Syntax**:
```aether
🛑 errorCode
```

**Example**:
```aether
🛑 404
🛑 "Not Found"
```

---

### ✓ - Success

**Symbol**: `✓` (Check Mark)  
**Unicode**: U+2713  
**Purpose**: Validate or mark success

**Syntax**:
```aether
✓ condition
```

**Example**:
```aether
✓ user
```
Validate user exists

---

### ◇ - If Conditional

**Symbol**: `◇` (White Diamond)  
**Unicode**: U+25C7  
**Purpose**: Conditional branching

**Syntax**:
```aether
◇ condition
```

---

### ⊕ - Logical OR

**Symbol**: `⊕` (Circled Plus)  
**Unicode**: U+2295  
**Purpose**: Logical OR operation

**Syntax**:
```aether
a ⊕ b
```

---

### ⊗ - Logical AND

**Symbol**: `⊗` (Circled Times)  
**Unicode**: U+2297  
**Purpose**: Logical AND operation

**Syntax**:
```aether
a ⊗ b
```

---

### ¬ - Logical NOT

**Symbol**: `¬` (Not Sign)  
**Unicode**: U+00AC  
**Purpose**: Logical negation

**Syntax**:
```aether
¬ condition
```

---

## Collections

### 🗂 - Array

**Symbol**: `🗂` (Card Index Dividers Emoji)  
**Unicode**: U+1F5C2  
**Purpose**: Array/list data structure

**Syntax**:
```aether
🗂 [elements]
```

---

### 🗄 - Map

**Symbol**: `🗄` (File Cabinet Emoji)  
**Unicode**: U+1F5C4  
**Purpose**: Map/dictionary data structure

**Syntax**:
```aether
🗄 {key: value}
```

---

### ∅ - Empty

**Symbol**: `∅` (Empty Set)  
**Unicode**: U+2205  
**Purpose**: Null/empty value

**Syntax**:
```aether
∅
```

---

## Literals & Types

### String Literals

**Syntax**:
```aether
"text content"
```

**Example**:
```aether
📤 "Hello, Aether!"
```

---

### Number Literals

**Syntax**:
```aether
42
3.14
-10
```

**Example**:
```aether
📤 200
```

---

### Identifiers

**Syntax**:
```aether
variable_name
userId
count
```

**Example**:
```aether
📥 ▷ user
💾 user
```

---

## Network Operations

### 🌐 - HTTP Request

**Symbol**: `🌐` (Globe with Meridians Emoji)  
**Unicode**: U+1F310  
**Purpose**: Make HTTP request

**Syntax**:
```aether
🌐 url
```

---

### ® - Register

**Symbol**: `®` (Registered Sign)  
**Unicode**: U+00AE  
**Purpose**: Register/create operation

**Syntax**:
```aether
ƒ®: body
```

**Example**:
```aether
ƒ®: 📥⇢J ▷ u ⨠ 💾u
```
Register function that persists user

---

## Complete Example

### User Registration Endpoint

```aether
ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200
```

**Breakdown**:
1. `ƒ®:` - Define function "register"
2. `📥⇢J` - Input piped to JSON parser
3. `▷ u` - Bind result to variable `u`
4. `⁇` - Guard: check if null
5. `🛑400` - If null, halt with 400 error
6. `⨠` - Then (sequence)
7. `💾u` - Persist variable `u`
8. `⨠` - Then
9. `📤200` - Output success code 200

**Equivalent Traditional Code**:
```javascript
function register(input) {
  const user = JSON.parse(input);
  if (!user) {
    throw new Error(400);
  }
  database.save(user);
  return 200;
}
```

---

## Tips for Using Symbols

### Input Methods

1. **Copy-Paste**: Copy symbols from this reference
2. **Unicode Input**: Use OS Unicode input (e.g., Alt codes on Windows)
3. **IDE Snippets**: Configure editor shortcuts
4. **IME**: Use Input Method Editor for Unicode
5. **Character Map**: Use system character map utility

### Symbol Discovery

- Use `aether symbols` command to view reference
- Check online Unicode databases
- Refer to this documentation

### Best Practices

1. **Consistency**: Use same symbols for same operations
2. **Clarity**: Prefer readable symbol chains
3. **Comments**: Document complex symbol sequences
4. **Spacing**: Use whitespace for readability
5. **Testing**: Verify symbol rendering in your editor

---

## Symbol Encoding

All symbols are UTF-8 encoded:
- **1 byte**: ASCII (J, S, N)
- **2 bytes**: Latin extended, Greek (ƒ, λ, ®)
- **3 bytes**: Mathematical operators (⇢, ▷, ⨠, ⁇, ◇, ⊕, ⊗, ¬, ∅)
- **4 bytes**: Emoji (📥, 📤, 💾, 🔍, 🛑, ✓, 🗂, 🗄, 🌐)

This encoding allows maximum information density while maintaining compatibility with modern text systems.

---

**Aether (以太)** - Where every symbol carries meaning, and every character counts.
