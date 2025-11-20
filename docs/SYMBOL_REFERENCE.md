# Aether Symbol Reference Guide

Complete reference for all symbols in the Aether programming language.

## 📑 Table of Contents

1. [Function & Control Flow](#function--control-flow)
2. [Data Operations](#data-operations)
3. [Logic & Conditionals](#logic--conditionals)
4. [Collections](#collections)
5. [Literals & Types](#literals--types)
6. [Network Operations](#network-operations)
7. [Control Flow & Iteration (v1.1)](#control-flow--iteration-v11)
8. [Concurrency & Async (v1.1)](#concurrency--async-v11)
9. [Data Manipulation (v1.1)](#data-manipulation-v11)
10. [System & Environment (v1.1)](#system--environment-v11)
11. [Complete Examples](#complete-example)

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

## Control Flow & Iteration (v1.1)

### ↻ - Loop

**Symbol**: `↻` (Clockwise Gapped Circle Arrow)  
**Unicode**: U+21BB  
**Purpose**: Create unbounded loop/while loop

**Syntax**:
```aether
↻ body
```

**Example**:
```aether
↻ 📥 ⨠ 💾
```
Loop that reads and persists input

---

### ∀ - ForEach/Map

**Symbol**: `∀` (For All / Universal Quantifier)  
**Unicode**: U+2200  
**Purpose**: Iterate over collection, apply operation to each element

**Syntax**:
```aether
∀(variable): body
```

**Example**:
```aether
🗂urls ▷ ∀(u): 🌐📥u
```
Iterate over URLs array and fetch each one

---

### ∃ - Filter/Find

**Symbol**: `∃` (There Exists / Existential Quantifier)  
**Unicode**: U+2203  
**Purpose**: Filter collection based on predicate

**Syntax**:
```aether
∃(predicate)
```

**Example**:
```aether
∃(res.ok)
```
Filter results where `ok` property is truthy

---

### ∑ - Reduce/Sum

**Symbol**: `∑` (Summation)  
**Unicode**: U+2211  
**Purpose**: Reduce/aggregate collection to single value

**Syntax**:
```aether
∑ operation
```

**Example**:
```aether
🗂numbers ▷ ∑
```
Sum all numbers in array

---

### 🛡 - Try/Rescue

**Symbol**: `🛡` (Shield)  
**Unicode**: U+1F6E1  
**Purpose**: Exception handling / safe execution block

**Syntax**:
```aether
🛡(body)
```

**Example**:
```aether
🛡(🌐📥url)
```
Try HTTP GET, catch errors

---

### ♻ - Retry

**Symbol**: `♻` (Recycling Symbol)  
**Unicode**: U+267B  
**Purpose**: Retry operation on failure

**Syntax**:
```aether
♻count: body
```

**Example**:
```aether
♻3: 🌐📥url
```
Retry HTTP GET up to 3 times

---

## Concurrency & Async (v1.1)

### ⚡ - Async/Trigger

**Symbol**: `⚡` (High Voltage)  
**Unicode**: U+26A1  
**Purpose**: Execute operation asynchronously

**Syntax**:
```aether
⚡ body
```

**Example**:
```aether
⚡ 🌐📥url
```
Async HTTP GET request

---

### ⏳ - Await

**Symbol**: `⏳` (Hourglass Not Done)  
**Unicode**: U+23F3  
**Purpose**: Wait for async operation to complete

**Syntax**:
```aether
⏳ expression
```

**Example**:
```aether
⏳ asyncResult
```
Await async result

---

### 🧵 - Thread/Task

**Symbol**: `🧵` (Thread/Spool)  
**Unicode**: U+1F9F5  
**Purpose**: Spawn concurrent thread/task

**Syntax**:
```aether
🧵 body
```

**Example**:
```aether
🧵 💾data
```
Persist data in background thread

---

### 🔒 - Mutex/Lock

**Symbol**: `🔒` (Lock)  
**Unicode**: U+1F512  
**Purpose**: Critical section protection

**Syntax**:
```aether
🔒 body
```

**Example**:
```aether
🔒 counter ⨠ counter+1
```
Thread-safe counter increment

---

### 📡 - Emit/Signal

**Symbol**: `📡` (Satellite Antenna)  
**Unicode**: U+1F4E1  
**Purpose**: Emit event/broadcast signal

**Syntax**:
```aether
📡 event
```

**Example**:
```aether
📡 "userCreated"
```
Emit user created event

---

### 👁 - Watch/Listen

**Symbol**: `👁` (Eye)  
**Unicode**: U+1F441  
**Purpose**: Listen to events/watch for changes

**Syntax**:
```aether
👁 event handler
```

**Example**:
```aether
👁 "userCreated" 🪵
```
Watch for user created events and log them

---

## Data Manipulation (v1.1)

### ✂ - Split/Slice

**Symbol**: `✂` (Scissors)  
**Unicode**: U+2702  
**Purpose**: Split string or slice array

**Syntax**:
```aether
✂ delimiter
```

**Example**:
```aether
"a,b,c" ▷ ✂","
```
Split string by comma

---

### 🔗 - Join/Concat

**Symbol**: `🔗` (Link)  
**Unicode**: U+1F517  
**Purpose**: Join array elements or concatenate

**Syntax**:
```aether
🔗 separator
```

**Example**:
```aether
🗂["a","b","c"] ▷ 🔗","
```
Join array elements with comma

---

### ✱ - Regex/Match

**Symbol**: `✱` (Heavy Asterisk)  
**Unicode**: U+2731  
**Purpose**: Regular expression pattern matching

**Syntax**:
```aether
✱ pattern
```

**Example**:
```aether
email ▷ ✱"@.*\\.com"
```
Match email pattern

---

### ≡ - Equal

**Symbol**: `≡` (Identical To)  
**Unicode**: U+2261  
**Purpose**: Strict equality comparison

**Syntax**:
```aether
value ≡ expected
```

**Example**:
```aether
status ≡ 200
```
Check if status equals 200

---

### ≠ - Not Equal

**Symbol**: `≠` (Not Equal To)  
**Unicode**: U+2260  
**Purpose**: Inequality comparison

**Syntax**:
```aether
value ≠ expected
```

**Example**:
```aether
status ≠ 404
```
Check if status is not 404

---

### 🧊 - Immutable/Const

**Symbol**: `🧊` (Ice Cube)  
**Unicode**: U+1F9CA  
**Purpose**: Define immutable constant

**Syntax**:
```aether
🧊 name value
```

**Example**:
```aether
🧊 MAX_RETRIES 3
```
Define immutable constant

---

## System & Environment (v1.1)

### 🧩 - Import/Module

**Symbol**: `🧩` (Puzzle Piece)  
**Unicode**: U+1F9E9  
**Purpose**: Import module/dependency

**Syntax**:
```aether
🧩 moduleName
```

**Example**:
```aether
🧩🌐
```
Import HTTP module

---

### 🔑 - Auth/Token

**Symbol**: `🔑` (Key)  
**Unicode**: U+1F511  
**Purpose**: Authentication/token injection

**Syntax**:
```aether
🔑 token
```

**Example**:
```aether
🔑 apiKey
```
Inject authentication token

---

### 📅 - Date/Time

**Symbol**: `📅` (Calendar)  
**Unicode**: U+1F4C5  
**Purpose**: Date/time operations

**Syntax**:
```aether
📅
```

**Example**:
```aether
📅 ▷ timestamp
```
Get current timestamp

---

### 🎲 - Random

**Symbol**: `🎲` (Game Die)  
**Unicode**: U+1F3B2  
**Purpose**: Random number generation

**Syntax**:
```aether
🎲
```

**Example**:
```aether
🎲 ▷ randomValue
```
Generate random number

---

### 🪵 - Log

**Symbol**: `🪵` (Wood)  
**Unicode**: U+1FAB5  
**Purpose**: Log message to console/file

**Syntax**:
```aether
🪵 message
```

**Example**:
```aether
🪵 "Processing complete"
```
Log message

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

### Concurrent Web Crawler (v1.1)

A real-world example demonstrating the new v1.1 features:

**Requirements:**
1. Import HTTP module
2. Define a URL list
3. Concurrently fetch each URL
4. If fetch fails, retry up to 3 times
5. Parse results and filter valid data
6. Save to database

**Aether v1.1 Implementation:**
```aether
🧩🌐 ⨠ 🗂urls ▷ ∀(u): (⚡ 🛡(♻3: 🌐📥u)) ▷ res ⁇ 🛑 ⨠ ∃(res.ok) ▷ 💾
```

**Breakdown:**
1. `🧩🌐` - Import HTTP module
2. `⨠` - Then (sequence)
3. `🗂urls` - Get URL array
4. `▷` - Pipe into...
5. `∀(u):` - For each URL `u`
6. `⚡` - Execute asynchronously
7. `🛡(...)` - Try/Catch block
8. `♻3:` - Retry up to 3 times
9. `🌐📥u` - HTTP GET URL `u`
10. `▷ res` - Bind result to `res`
11. `⁇ 🛑` - Guard: if null, halt this task
12. `⨠` - Then
13. `∃(res.ok)` - Filter only successful results
14. `▷ 💾` - Pipe to database persist

**Equivalent Traditional Code:**
```javascript
import http from 'http-module';

const urls = [...];

await Promise.all(
  urls.map(async (u) => {
    let res;
    for (let i = 0; i < 3; i++) {
      try {
        res = await http.get(u);
        if (res) break;
      } catch (e) {
        if (i === 2) return null;
      }
    }
    
    if (!res) return;
    
    if (res.ok) {
      await database.save(res);
    }
  })
);
```

**Token Comparison:**
- Traditional JavaScript: ~300+ characters
- Aether v1.1: ~70 characters
- **Compression ratio: 4.3x**

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
- **3 bytes**: Mathematical operators (⇢, ▷, ⨠, ⁇, ◇, ⊕, ⊗, ¬, ∅, ∀, ∃, ∑, ↻, ≡, ≠, ✂, ✱)
- **4 bytes**: Emoji (📥, 📤, 💾, 🔍, 🛑, ✓, 🗂, 🗄, 🌐, 🛡, ♻, ⚡, ⏳, 🧵, 🔒, 📡, 👁, 🔗, 🧊, 🧩, 🔑, 📅, 🎲, 🪵)

**Aether v1.1** now includes:
- **Control Flow**: 6 new symbols for loops, iteration, and error handling
- **Concurrency**: 6 new symbols for async/await and parallel execution
- **Data Operations**: 6 new symbols for string/array manipulation and comparisons
- **System/Environment**: 5 new symbols for modules, auth, time, random, and logging

This encoding allows maximum information density while maintaining compatibility with modern text systems.

---

**Aether (以太)** - Where every symbol carries meaning, and every character counts.
