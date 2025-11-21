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
11. [Testing & Debugging (v1.2)](#testing--debugging-v12)
12. [Security & Crypto (v1.2)](#security--crypto-v12)
13. [Math & Science (v1.2)](#math--science-v12)
14. [File System (v1.3)](#file-system-v13)
15. [Streams & Buffers (v1.3)](#streams--buffers-v13)
16. [Networking (v1.3)](#networking-v13)
17. [Process & OS (v1.3)](#process--os-v13)
18. [Standard Library (v1.4)](#standard-library-v14)
19. [Complete Examples](#complete-example)

## Standard Library (v1.4)

For standard library modules and their specialized operators, see the comprehensive [Standard Library Documentation](STANDARD_LIBRARY.md).

Quick reference of standard library modules:
- **🧩🧮** Math - Mathematics & Statistics (π, e, x̄, σ, ∫, ∂, M)
- **🧩📝** Text - Text & Encoding (🔡, 🔠, 📋, 6️⃣4️⃣, 🆔)
- **🧩⏰** Chrono - Time & Scheduling (⏳, 🗓️, ⏰, ∆t)
- **🧩🗜️** Compression - Compression & Archiving (📦, 📤, ⚖️)
- **🧩🧠** AI & Tensor - Cognition & Tensors (▦, 🕸️, 🤖, 🏋️)
- **🧩🌐** Net+ - Advanced Networking (🔌⇄, 📧, ☁️)

---

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

## Testing & Debugging (v1.2)

### 🧪 - Test/Suite

**Symbol**: `🧪` (Test Tube)  
**Unicode**: U+1F9EA  
**Purpose**: Define test case or test suite

**Syntax**:
```aether
🧪 "TestName": body
```

**Example**:
```aether
🧪 "AuthTest": 🎭💾 ⨠ "admin" ⇢ ® ▷ res ⨠ ⚖️(res ≡ 200)
```
Define a test suite for authentication

---

### ⚖️ - Assert

**Symbol**: `⚖️` (Balance Scale)  
**Unicode**: U+2696 + U+FE0F  
**Purpose**: Assert condition - throws error if condition is false

**Syntax**:
```aether
⚖️(condition)
```

**Example**:
```aether
⚖️(status ≡ 200)
⚖️(time < 50ms)
```
Assert that status equals 200 and time is less than 50ms

---

### 🎭 - Mock/Stub

**Symbol**: `🎭` (Performing Arts)  
**Unicode**: U+1F3AD  
**Purpose**: Mock external dependencies or data for testing

**Syntax**:
```aether
🎭 dependency
```

**Example**:
```aether
🎭💾
```
Mock database operations

---

### ⏱️ - Benchmark

**Symbol**: `⏱️` (Stopwatch)  
**Unicode**: U+23F1 + U+FE0F  
**Purpose**: Measure execution time and performance analysis

**Syntax**:
```aether
⏱️(body) ▷ timeVar
```

**Example**:
```aether
⏱️("admin" ⇢ ®) ▷ time ⨠ ⚖️(time < 50ms)
```
Benchmark registration function and assert it completes in under 50ms

---

### 🐛 - Debug

**Symbol**: `🐛` (Bug)  
**Unicode**: U+1F41B  
**Purpose**: Enter debug mode or set breakpoint

**Syntax**:
```aether
🐛
```

**Example**:
```aether
📥 ⨠ 🐛 ⨠ 💾
```
Set breakpoint between input and persist operations

---

## Security & Crypto (v1.2)

### 🔐 - Encrypt

**Symbol**: `🔐` (Closed Lock with Key)  
**Unicode**: U+1F510  
**Purpose**: Encrypt data using public key or symmetric key

**Syntax**:
```aether
data ⇢ 🔐 key
```

**Example**:
```aether
message ⇢ 🔐 publicKey ▷ encrypted
```
Encrypt message with public key

---

### 🔓 - Decrypt

**Symbol**: `🔓` (Open Lock)  
**Unicode**: U+1F513  
**Purpose**: Decrypt encrypted data

**Syntax**:
```aether
encrypted ⇢ 🔓 key
```

**Example**:
```aether
encrypted ⇢ 🔓 privateKey ▷ plaintext
```
Decrypt data with private key

---

### #️⃣ - Hash

**Symbol**: `#️⃣` (Number Sign/Hash Key)  
**Unicode**: U+0023 + U+FE0F + U+20E3  
**Purpose**: Calculate hash value (SHA, MD5, etc.)

**Syntax**:
```aether
data ⇢ #️⃣
```

**Example**:
```aether
(password 🔗 salt) ⇢ #️⃣ ▷ hash
```
Hash password concatenated with salt

---

### ✍️ - Sign

**Symbol**: `✍️` (Writing Hand)  
**Unicode**: U+270D + U+FE0F  
**Purpose**: Create digital signature

**Syntax**:
```aether
data ⇢ ✍️ privateKey
```

**Example**:
```aether
document ⇢ ✍️ signingKey ▷ signature
```
Sign document with private key

---

### 🛡️ - Verify

**Symbol**: `🛡️` (Shield) *with variation selector*  
**Unicode**: U+1F6E1 + U+FE0F  
**Purpose**: Verify digital signature (different from 🛡 Try)

**Syntax**:
```aether
🛡️(signature, data, publicKey)
```

**Example**:
```aether
🛡️(sig, doc, pubKey) ⨠ ✓
```
Verify signature and return success

**Note**: This is different from `🛡` (Try/Rescue) which is for exception handling.

---

## Math & Science (v1.2)

### ↑ - Power

**Symbol**: `↑` (Upwards Arrow)  
**Unicode**: U+2191  
**Purpose**: Power/exponentiation operation

**Syntax**:
```aether
base↑exponent
```

**Example**:
```aether
2↑3 ▷ result  // result = 8
(v1 - v2)↑2   // square the difference
```

---

### √ - Root

**Symbol**: `√` (Square Root)  
**Unicode**: U+221A  
**Purpose**: Square root operation

**Syntax**:
```aether
value ⇢ √
```

**Example**:
```aether
((v1 - v2)↑2) ⇢ √ ▷ dist
```
Calculate distance between vectors

---

### ≈ - Approx

**Symbol**: `≈` (Almost Equal To)  
**Unicode**: U+2248  
**Purpose**: Approximate equality for floating-point comparisons

**Syntax**:
```aether
value1 ≈ value2
```

**Example**:
```aether
◇(dist ≈ 0) 📤"Same"
```
Check if distance is approximately zero

---

### ∞ - Infinity

**Symbol**: `∞` (Infinity)  
**Unicode**: U+221E  
**Purpose**: Represent infinity value

**Syntax**:
```aether
∞
```

**Example**:
```aether
◇(count < ∞) ↻ process
```
Loop while count is less than infinity (always true)

---

### ∆ - Delta

**Symbol**: `∆` (Increment/Delta)  
**Unicode**: U+2206  
**Purpose**: Change amount or difference value

**Syntax**:
```aether
∆ variable
```

**Example**:
```aether
newValue - oldValue ▷ ∆temp
```
Calculate temperature change

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

### Secure Password Storage (v1.2 - Security)

A real-world example demonstrating v1.2 Security features:

**Requirements:**
1. Receive password input
2. Generate random salt value
3. Concatenate password with salt
4. Calculate hash
5. Store hash and salt in database
6. Return success

**Aether v1.2 Implementation:**
```aether
ƒ®: 📥pass ▷ p ⨠ 🎲 ▷ salt ⨠ (p 🔗 salt) ⇢ #️⃣ ▷ hash ⨠ 💾{h:hash, s:salt} ⨠ 📤✓
```

**Breakdown:**
1. `ƒ®:` - Define register function
2. `📥pass ▷ p` - Input password and bind to `p`
3. `⨠` - Then (sequence)
4. `🎲 ▷ salt` - Generate random salt value
5. `⨠` - Then
6. `(p 🔗 salt)` - Concatenate password with salt
7. `⇢ #️⃣` - Pipe through hash function
8. `▷ hash` - Bind result to `hash`
9. `⨠` - Then
10. `💾{h:hash, s:salt}` - Persist hash and salt object
11. `⨠` - Then
12. `📤✓` - Return success

**Equivalent Traditional Code:**
```javascript
function register(password) {
  const p = password;
  const salt = Math.random().toString(36);
  const combined = p + salt;
  const hash = crypto.createHash('sha256').update(combined).digest('hex');
  await database.save({ h: hash, s: salt });
  return true;
}
```

---

### Unit Testing with Mocks (v1.2 - Testing)

A testing example demonstrating v1.2 Testing features:

**Requirements:**
1. Define test suite named "AuthTest"
2. Mock database layer
3. Benchmark the registration function call
4. Assert result status is 200
5. Assert execution time is less than 50ms

**Aether v1.2 Implementation:**
```aether
🧪 "AuthTest": 🎭💾 ⨠ ⏱️("admin" ⇢ ® ▷ res) ▷ time ⨠ ⚖️(res ≡ 200) ⨠ ⚖️(time < 50ms)
```

**Breakdown:**
1. `🧪 "AuthTest":` - Define test suite
2. `🎭💾` - Mock the database operations
3. `⨠` - Then
4. `⏱️(...)` - Start benchmark timer
5. `"admin" ⇢ ®` - Call register with "admin"
6. `▷ res` - Bind result
7. `▷ time` - Bind execution time
8. `⨠` - Then
9. `⚖️(res ≡ 200)` - Assert result equals 200
10. `⨠` - Then
11. `⚖️(time < 50ms)` - Assert time less than 50ms

**Equivalent Traditional Code:**
```javascript
describe("AuthTest", () => {
  beforeEach(() => {
    mockDatabase();
  });

  it("should complete in under 50ms", async () => {
    const startTime = performance.now();
    const res = await register("admin");
    const time = performance.now() - startTime;
    
    expect(res).toBe(200);
    expect(time).toBeLessThan(50);
  });
});
```

---

### Vector Distance Calculation (v1.2 - Math)

A scientific computing example demonstrating v1.2 Math features:

**Requirements:**
1. Calculate distance between two vectors
2. Use approximate equality for floating-point comparison
3. Return "Same" if vectors are approximately equal

**Aether v1.2 Implementation:**
```aether
ƒ calc: 📥v1 📥v2 ⨠ ((v1 - v2)↑2) ⇢ √ ▷ dist ⨠ ◇(dist ≈ 0) 📤"Same"
```

**Breakdown:**
1. `ƒ calc:` - Define calc function
2. `📥v1 📥v2` - Input two vectors
3. `⨠` - Then
4. `(v1 - v2)` - Subtract vectors
5. `↑2` - Square the result (power of 2)
6. `⇢ √` - Pipe to square root
7. `▷ dist` - Bind to distance variable
8. `⨠` - Then
9. `◇(dist ≈ 0)` - If distance approximately equals 0
10. `📤"Same"` - Output "Same"

**Equivalent Traditional Code:**
```javascript
function calc(v1, v2) {
  const diff = v1 - v2;
  const squared = Math.pow(diff, 2);
  const dist = Math.sqrt(squared);
  
  if (Math.abs(dist - 0) < Number.EPSILON) {
    return "Same";
  }
}
```

---

## File System (v1.3)

### 📄 - File

**Symbol**: `📄` (File Emoji)  
**Unicode**: U+1F4C4  
**Purpose**: File handle/object representation

**Syntax**:
```aether
📄📍"path"
```

**Example**:
```aether
📄📍"/var/log/app.log" ▷ f
```
Create file handle for the specified path

---

### 📂 - Dir

**Symbol**: `📂` (Folder Emoji)  
**Unicode**: U+1F4C2  
**Purpose**: Directory/folder representation

**Syntax**:
```aether
📂📍"path"
```

**Example**:
```aether
📂📍"/tmp" ▷ dir
```
Create directory handle

---

### 📍 - Path

**Symbol**: `📍` (Round Pushpin Emoji)  
**Unicode**: U+1F4CD  
**Purpose**: Path resolution and manipulation

**Syntax**:
```aether
📍"path/to/resource"
```

**Example**:
```aether
📍"/etc/config.json" ▷ path
```
Resolve and validate path

---

### 📖 - Read

**Symbol**: `📖` (Open Book Emoji)  
**Unicode**: U+1F4D6  
**Purpose**: Read content from file or stream

**Syntax**:
```aether
source ⇢ 📖
```

**Example**:
```aether
📄📍"data.txt" ⇢ 📖 ▷ content
```
Read content from file

---

### 🖊️ - Write

**Symbol**: `🖊️` (Pen Emoji)  
**Unicode**: U+1F58A + U+FE0F  
**Purpose**: Write content to file (overwrite mode)

**Syntax**:
```aether
content ⇢ 🖊️target
```

**Example**:
```aether
"Hello World" ⇢ 🖊️📄📍"output.txt"
```
Write string to file, overwriting existing content

---

### 🖇️ - Append

**Symbol**: `🖇️` (Linked Paperclips Emoji)  
**Unicode**: U+1F587 + U+FE0F  
**Purpose**: Append content to file

**Syntax**:
```aether
content ⇢ 🖇️target
```

**Example**:
```aether
"New log entry" ⇢ 🖇️📄📍"app.log"
```
Append content to existing file

---

### 🗑️ - Delete

**Symbol**: `🗑️` (Wastebasket Emoji)  
**Unicode**: U+1F5D1 + U+FE0F  
**Purpose**: Delete file or resource

**Syntax**:
```aether
🗑️target
```

**Example**:
```aether
🗑️📄📍"temp.txt"
```
Delete specified file

---

### 🛂 - Perm

**Symbol**: `🛂` (Passport Control Emoji)  
**Unicode**: U+1F6C2  
**Purpose**: Permission control (chmod/chown equivalent)

**Syntax**:
```aether
🛂(target, permission)
```

**Example**:
```aether
🛂(📄📍"script.sh", 755)
```
Set file permissions

---

## Streams & Buffers (v1.3)

### 🌊 - Stream

**Symbol**: `🌊` (Water Wave Emoji)  
**Unicode**: U+1F30A  
**Purpose**: Data stream (Readable/Writable Stream)

**Syntax**:
```aether
source ⇢ 🌊
```

**Example**:
```aether
📄📍"large-file.dat" ⇢ 🌊 ▷ stream
```
Create stream from file for efficient processing

---

### 🧱 - Buffer

**Symbol**: `🧱` (Brick Emoji)  
**Unicode**: U+1F9F1  
**Purpose**: Binary buffer (Bytes/Blob)

**Syntax**:
```aether
🧱size
```

**Example**:
```aether
🧱4KB ▷ buffer
```
Allocate 4KB buffer for data

---

### 🌬️ - Flush

**Symbol**: `🌬️` (Wind Face Emoji)  
**Unicode**: U+1F32C + U+FE0F  
**Purpose**: Flush buffer to ensure data is written

**Syntax**:
```aether
target ⇢ 🌬️
```

**Example**:
```aether
stream ⇢ 🌬️
```
Flush stream buffer

---

### 🔚 - EOF

**Symbol**: `🔚` (END Arrow Emoji)  
**Unicode**: U+1F51A  
**Purpose**: End of file/stream marker

**Syntax**:
```aether
stream ≠ 🔚
```

**Example**:
```aether
↻(stream ≠ 🔚): (stream ⇢ 📖 ▷ data)
```
Loop until end of stream

---

### ⏭️ - Skip/Seek

**Symbol**: `⏭️` (Next Track Button Emoji)  
**Unicode**: U+23ED + U+FE0F  
**Purpose**: Skip bytes or move stream pointer

**Syntax**:
```aether
⏭️count
```

**Example**:
```aether
stream ⇢ ⏭️1024
```
Skip 1024 bytes in stream

---

## Networking (v1.3)

### 🔌 - Socket

**Symbol**: `🔌` (Electric Plug Emoji)  
**Unicode**: U+1F50C  
**Purpose**: Network socket (TCP/UDP)

**Syntax**:
```aether
🔌protocol
```

**Example**:
```aether
🔌TCP ▷ socket
```
Create TCP socket

---

### 👂 - Listen

**Symbol**: `👂` (Ear Emoji)  
**Unicode**: U+1F442  
**Purpose**: Listen on port (Server Bind)

**Syntax**:
```aether
👂port
```

**Example**:
```aether
👂8080 ▷ listener
```
Listen for connections on port 8080

---

### 📞 - Connect

**Symbol**: `📞` (Telephone Receiver Emoji)  
**Unicode**: U+1F4DE  
**Purpose**: Initiate connection (Client Connect)

**Syntax**:
```aether
📞address
```

**Example**:
```aether
📞"localhost:8080" ▷ conn
```
Connect to remote server

---

### 🚪 - Port

**Symbol**: `🚪` (Door Emoji)  
**Unicode**: U+1F6AA  
**Purpose**: Port number specification

**Syntax**:
```aether
🚪number
```

**Example**:
```aether
🚪3000
```
Specify port 3000

---

### 📦 - Packet

**Symbol**: `📦` (Package Emoji)  
**Unicode**: U+1F4E6  
**Purpose**: Data packet (Datagram)

**Syntax**:
```aether
📦data
```

**Example**:
```aether
📦"Hello" ⇢ socket
```
Create and send packet

---

### 🤝 - Handshake

**Symbol**: `🤝` (Handshake Emoji)  
**Unicode**: U+1F91D  
**Purpose**: Protocol handshake/establish connection

**Syntax**:
```aether
🤝connection
```

**Example**:
```aether
conn ⇢ 🤝
```
Perform connection handshake

---

## Process & OS (v1.3)

### ⚙️ - Process

**Symbol**: `⚙️` (Gear Emoji)  
**Unicode**: U+2699 + U+FE0F  
**Purpose**: Process object and management

**Syntax**:
```aether
⚙️command
```

**Example**:
```aether
⚙️"python script.py" ▷ proc
```
Create process for command

---

### 🐚 - Shell

**Symbol**: `🐚` (Spiral Shell Emoji)  
**Unicode**: U+1F41A  
**Purpose**: Execute shell command

**Syntax**:
```aether
🐚"command"
```

**Example**:
```aether
🐚"ls -la" ▷ output
```
Execute shell command and capture output

---

### 🌍 - Env

**Symbol**: `🌍` (Earth Globe Emoji)  
**Unicode**: U+1F30D  
**Purpose**: Environment variable (Get/Set)

**Syntax**:
```aether
🌍"VAR_NAME"
```

**Example**:
```aether
🌍"PATH" ▷ path
```
Get PATH environment variable

---

### 🐏 - Memory

**Symbol**: `🐏` (Ram Emoji)  
**Unicode**: U+1F40F  
**Purpose**: Memory operations/manual allocation

**Syntax**:
```aether
🐏size
```

**Example**:
```aether
🐏1MB ▷ mem
```
Allocate 1MB of memory

---

### 👋 - Exit

**Symbol**: `👋` (Waving Hand Emoji)  
**Unicode**: U+1F44B  
**Purpose**: Exit program with exit code

**Syntax**:
```aether
👋code
```

**Example**:
```aether
👋0
```
Exit program successfully

---

### 📶 - Signal

**Symbol**: `📶` (Antenna Bars Emoji)  
**Unicode**: U+1F4F6  
**Purpose**: Send/capture system signal

**Syntax**:
```aether
📶signal
```

**Example**:
```aether
📶SIGTERM ⇢ process
```
Send SIGTERM signal to process

---

## Complete v1.3 Examples

### High-Performance Log Rotation

A real-world example demonstrating v1.3 File System features:

**Requirements:**
1. Accept log message as input
2. Open log file handle
3. Check if file size exceeds 1GB
4. If too large, rotate log file using shell command
5. Append message to log file

**Aether v1.3 Implementation:**
```aether
ƒ log: 📥msg ⨠ 📄📍"/var/log/app.log" ▷ f ⨠ ◇(f.size > 1GB): 🐚"mv /var/log/app.log /var/log/app.old" ⨠ msg ⇢ 🖇️f
```

**Breakdown:**
1. `ƒ log:` - Define log function
2. `📥msg` - Input message parameter
3. `⨠` - Then (sequence)
4. `📄📍"/var/log/app.log"` - Create file handle for log path
5. `▷ f` - Bind to variable `f`
6. `⨠` - Then
7. `◇(f.size > 1GB):` - If file size exceeds 1GB
8. `🐚"mv /var/log/app.log /var/log/app.old"` - Execute shell command to rotate
9. `⨠` - Then
10. `msg ⇢ 🖇️f` - Append message to file

**Equivalent Traditional Code:**
```javascript
function log(msg) {
  const f = openFile("/var/log/app.log");
  if (f.size > 1024 * 1024 * 1024) {
    exec("mv /var/log/app.log /var/log/app.old");
  }
  f.append(msg);
}
```

---

### TCP Echo Server

A networking example demonstrating v1.3 Socket programming:

**Requirements:**
1. Create TCP socket
2. Listen on port 8080
3. Accept connections in infinite loop
4. For each connection, handle asynchronously:
   - Create stream from connection
   - Read data until EOF
   - Echo data back
   - Flush buffer
5. Close connection

**Aether v1.3 Implementation:**
```aether
🔌TCP ⨠ 👂8080 ⨠ ↻: (⏳👂 ▷ conn ⨠ ⚡(🛡(conn ⇢ 🌊 ▷ s ⨠ ↻(s ≠ 🔚): (s ⇢ 📖 ▷ data ⨠ data ⇢ 🖊️s ⨠ s ⇢ 🌬️)) ⨠ conn ⇢ 👋))
```

**Breakdown:**
1. `🔌TCP` - Create TCP socket
2. `⨠` - Then
3. `👂8080` - Listen on port 8080
4. `⨠` - Then
5. `↻:` - Infinite loop
6. `⏳👂` - Await new connection
7. `▷ conn` - Bind connection to `conn`
8. `⨠` - Then
9. `⚡(...)` - Handle asynchronously (Go-style)
10. `🛡(...)` - Error protection
11. `conn ⇢ 🌊` - Create stream from connection
12. `▷ s` - Bind to `s`
13. `⨠` - Then
14. `↻(s ≠ 🔚):` - Loop until stream end
15. `s ⇢ 📖` - Read from stream
16. `▷ data` - Bind data
17. `⨠` - Then
18. `data ⇢ 🖊️s` - Write data back to stream
19. `⨠` - Then
20. `s ⇢ 🌬️` - Flush buffer
21. `⨠ conn ⇢ 👋` - Close connection

**Equivalent Traditional Code:**
```go
func server() {
  socket := createSocket("TCP")
  listener := socket.listen(8080)
  
  for {
    conn := listener.accept()
    
    go func() {
      defer conn.close()
      
      stream := conn.getStream()
      for !stream.eof() {
        data := stream.read()
        stream.write(data)
        stream.flush()
      }
    }()
  }
}
```

---

### Stream Processing Large Files

A streams example demonstrating efficient large file processing:

**Requirements:**
1. Open file as stream
2. Create 4KB buffer
3. Loop until EOF
4. Read chunks and split by newlines
5. Process each line (save to database)

**Aether v1.3 Implementation:**
```aether
📄📍"./data.csv" ⇢ 🌊 ▷ stream ⨠ ↻(stream ≠ 🔚): (stream ⇢ 📖(🧱4KB) ▷ chunk ⨠ chunk ⇢ ✂"\n" ▷ lines ⨠ ∀(lines): 💾)
```

**Breakdown:**
1. `📄📍"./data.csv"` - File handle for data.csv
2. `⇢ 🌊` - Create stream
3. `▷ stream` - Bind to stream variable
4. `⨠` - Then
5. `↻(stream ≠ 🔚):` - Loop until end of file
6. `stream ⇢ 📖(🧱4KB)` - Read 4KB chunk
7. `▷ chunk` - Bind chunk
8. `⨠` - Then
9. `chunk ⇢ ✂"\n"` - Split by newlines
10. `▷ lines` - Bind lines array
11. `⨠` - Then
12. `∀(lines): 💾` - For each line, persist to database

**Equivalent Traditional Code:**
```python
with open("./data.csv", "r") as file:
  while True:
    chunk = file.read(4096)
    if not chunk:
      break
    lines = chunk.split("\n")
    for line in lines:
      database.save(line)
```

---

### Environment Variables and Shell Integration

A process/OS example demonstrating system integration:

**Requirements:**
1. Get PATH environment variable
2. Execute shell command
3. Output result

**Aether v1.3 Implementation:**
```aether
🌍"PATH" ▷ path ⨠ 🐚"ls -la" ▷ output ⨠ 📤output
```

**Breakdown:**
1. `🌍"PATH"` - Get PATH environment variable
2. `▷ path` - Bind to path variable
3. `⨠` - Then
4. `🐚"ls -la"` - Execute shell command
5. `▷ output` - Bind output
6. `⨠` - Then
7. `📤output` - Output result

**Equivalent Traditional Code:**
```javascript
const path = process.env.PATH;
const output = exec("ls -la");
console.log(output);
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
- **3 bytes**: Mathematical operators (⇢, ▷, ⨠, ⁇, ◇, ⊕, ⊗, ¬, ∅, ∀, ∃, ∑, ↻, ≡, ≠, ✂, ✱, ↑, √, ≈, ∞, ∆)
- **4 bytes**: Emoji (📥, 📤, 💾, 🔍, 🛑, ✓, 🗂, 🗄, 🌐, 🛡, ♻, ⚡, ⏳, 🧵, 🔒, 📡, 👁, 🔗, 🧊, 🧩, 🔑, 📅, 🎲, 🪵, 🧪, 🎭, 🐛, 🔐, 🔓)
- **Multi-byte emoji sequences**: (⚖️, ⏱️, #️⃣, ✍️, 🛡️)

**Aether v1.1** includes:
- **Control Flow**: 6 new symbols for loops, iteration, and error handling
- **Concurrency**: 6 new symbols for async/await and parallel execution
- **Data Operations**: 6 new symbols for string/array manipulation and comparisons
- **System/Environment**: 5 new symbols for modules, auth, time, random, and logging

**Aether v1.2** adds:
- **Testing & Debugging**: 5 new symbols for test definitions, assertions, mocks, benchmarking, and debugging
- **Security & Crypto**: 5 new symbols for encryption, decryption, hashing, signing, and verification
- **Math & Science**: 5 new symbols for power operations, roots, approximate equality, infinity, and delta calculations

**Aether v1.3** adds:
- **File System**: 8 new symbols for file operations, directories, paths, reading, writing, appending, deleting, and permissions
- **Streams & Buffers**: 5 new symbols for stream processing, buffers, flushing, EOF detection, and seeking
- **Networking**: 6 new symbols for sockets, listening, connecting, ports, packets, and handshakes
- **Process & OS**: 6 new symbols for process management, shell execution, environment variables, memory allocation, exit, and signals

This encoding allows maximum information density while maintaining compatibility with modern text systems.

---

**Aether (以太)** - Where every symbol carries meaning, and every character counts.
