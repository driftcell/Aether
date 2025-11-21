# Aether Standard Library (ASL)

## Overview

The **Aether Standard Library (ASL)** represents **经过高度优化的通用能力 (highly optimized universal capabilities)**. In Aether's vision, standard library modules are not just code collections—they represent fundamental capabilities that typically map directly to underlying C++/Rust implementations or even hardware-accelerated instructions.

## Design Philosophy

Aether's standard library follows the **"Import as Capability"** principle:
- Import modules using the `🧩` (Puzzle Piece) emoji
- Gain access to high-level operators for that domain
- All functions are **stateless** and **composable**
- Perfect integration with Aether's `⇢` (Pipe) philosophy
- **High Leverage**: Encapsulates commonly needed operations that would be verbose with basic symbols

## Module Import Mechanism

**Syntax**: `🧩[ModuleEmoji]`

After importing, the module's specialized symbols and functions become available in the current scope.

**Example**:
```aether
🧩🧮 ⨠ 2*π ▷ circumference
```
Import the Math module and use the π constant.

---

## Core Standard Library Modules

## 🧩🧮 Math (Mathematics & Statistics)

Provides scientific computing capabilities beyond basic arithmetic operators.

### Constants & Variables

| Symbol | Description | Example |
|--------|-------------|---------|
| `π` | Pi constant (3.14159...) | `🧩🧮 ⨠ 2*π` |
| `e` | Euler's number (2.71828...) | `🧩🧮 ⨠ e↑2` |

### Statistical Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `x̄` | Mean (Average) | `🗂data ⇢ x̄ ▷ avg` |
| `σ` | Standard Deviation | `🗂data ⇢ σ ▷ stddev` |

### Calculus Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `∫` | Integrate (definite integral) | `ƒ ⇢ ∫(0,10) ▷ area` |
| `∂` | Derive (derivative) | `ƒ ⇢ ∂ ▷ derivative` |

### Matrix Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `M` | Matrix Operations | `M[2,2]` (define 2x2 matrix) |

### Complete Example

**Task**: Read CSV file, calculate average of column 2, format output

```aether
🧩🧮 🧩📝 ⨠
📄📍"data.csv" ⇢ 🌊 ⇢ ✂"\n" ▷ rows ⨠
rows ⇢ ∀(r): (r ⇢ ✂"," ▷ cols ⇢ cols[2]) ▷ values ⨠
values ⇢ x̄ ▷ avg ⨠
"Average: {}" ⇢ 📋avg ⇢ 📤
```

**Breakdown**:
1. Import Math (🧩🧮) and Text (🧩📝) modules
2. Read CSV file and create stream
3. Split by newlines to get rows
4. Map each row: split by comma and get column 2
5. Calculate mean (x̄) of values
6. Format with template and output

---

## 🧩📝 Text (Text & Encoding)

Handles Unicode processing, encoding conversion, and text templates.

### Case Conversion

| Symbol | Description | Example |
|--------|-------------|---------|
| `🔡` | ToLowerCase | `"ABC" ⇢ 🔡 ▷ "abc"` |
| `🔠` | ToUpperCase | `"abc" ⇢ 🔠 ▷ "ABC"` |

### Template & Formatting

| Symbol | Description | Example |
|--------|-------------|---------|
| `📋` | Format/Template | `"Hi {}" ⇢ 📋name` |

### Encoding Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `6️⃣4️⃣` | Base64 Encode/Decode | `data ⇢ 6️⃣4️⃣ ▷ encoded` |

### UUID & Identifiers

| Symbol | Description | Example |
|--------|-------------|---------|
| `🆔` | UUID Generate | `🆔 ▷ id` |

### Complete Example

**Task**: Generate unique ID, format welcome message

```aether
🧩📝 ⨠
🆔 ▷ userId ⨠
"Welcome, user {}" ⇢ 📋userId ▷ message ⨠
message ⇢ 🔠 ⇢ 📤
```

**Breakdown**:
1. Import Text module
2. Generate UUID and bind to userId
3. Format template with userId
4. Convert to uppercase and output

---

## 🧩⏰ Chrono (Time & Scheduling)

Provides powerful time handling beyond the basic `📅` DateTime symbol.

### Time Control

| Symbol | Description | Example |
|--------|-------------|---------|
| `⏳` | Delay/Sleep | `⏳5s` (pause 5 seconds) |

### Date Formatting

| Symbol | Description | Example |
|--------|-------------|---------|
| `🗓️` | Format Date | `📅 ⇢ 🗓️"YYYY-MM-DD"` |

### Scheduling

| Symbol | Description | Example |
|--------|-------------|---------|
| `⏰` | Schedule/Cron | `⏰"0 0 * * *" ƒbackup` |

### Duration & Diff

| Symbol | Description | Example |
|--------|-------------|---------|
| `∆t` | Duration/Diff | `t2 - t1 ▷ ∆t` |

### Complete Example

**Task**: Daily database backup at midnight

```aether
🧩⏰ ⨠
⏰"0 0 * * *": (
  🐚"pg_dump db" ⇢ 📦 ▷ backup ⨠
  backup ⇢ 📄📍(📅 ⇢ 🗓️"backup-YYYYMMDD.sql.gz")
)
```

**Breakdown**:
1. Import Chrono module
2. Schedule cron job for midnight (0 0 * * *)
3. Execute shell command to dump database
4. Compress the dump (📦 is also from Compression module)
5. Save to file with formatted date in filename

---

## 🧩🗜️ Compression (Compression & Archiving)

Handles data stream compression and decompression.

### Compression Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `📦` | Pack (Zip/Gzip) | `file ⇢ 📦 ▷ compressed` |
| `📤` | Unpack (Unzip) | `compressed ⇢ 📤 ▷ data` |

**Note**: Within the Compression module context, `📤` takes on the meaning of Unpack/Decompress. In base Aether, `📤` means Output/Return. The context (whether the Compression module is imported) determines the behavior.

### Size Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `⚖️` | SizeOf | `data ⇢ ⚖️ ▷ size` |

**Note**: `⚖️` means SizeOf in the Compression module. In the base language (v1.2), `⚖️` is used for Assert in testing contexts. Context determines meaning.

### Complete Example

**Task**: Compress file, check size reduction

```aether
🧩🗜️ ⨠
📄📍"large.txt" ⇢ 📖 ▷ original ⨠
original ⇢ ⚖️ ▷ originalSize ⨠
original ⇢ 📦 ▷ compressed ⨠
compressed ⇢ ⚖️ ▷ compressedSize ⨠
"Reduced from {} to {}" ⇢ 📋(originalSize, compressedSize) ⇢ 📤
```

**Breakdown**:
1. Import Compression module
2. Read large.txt file
3. Get original file size
4. Compress the file
5. Get compressed size
6. Format and output comparison

---

## 🧩🧠 AI & Tensor (Cognition & Tensor Operations)

Aether as an AI-native language provides built-in AI capabilities.

### Tensor Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `▦` | Tensor/Matrix | `▦[1,0,1]` |

### Embedding Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `🕸️` | Embedding | `"text" ⇢ 🕸️ ▷ vector` |

### AI Inference

| Symbol | Description | Example |
|--------|-------------|---------|
| `🤖` | Inference (LLM) | `"prompt" ⇢ 🤖 ▷ response` |

### Training Operations

| Symbol | Description | Example |
|--------|-------------|---------|
| `🏋️` | Train/Fit | `data ⇢ 🏋️model ▷ trained` |

### Complete Example

**Task**: Smart image processing service - AI recognition, compression, cloud storage

```aether
🧩🧠 🧩🗜️ 🧩🌐 ⨠
ƒ handle: 📥img ⨠
  (img ⇢ 🤖"describe") ▷ desc ⨠  // AI recognition
  img ⇢ 📦 ▷ zip ⨠               // Gzip compression
  zip ⇢ ☁️"s3://images" ⨠        // Store in S3
  📤desc                         // Return description
```

**Breakdown**:
1. Import AI, Compression, and Net+ modules
2. Define handle function with image input
3. Use AI inference (🤖) to describe image
4. Compress image with Gzip
5. Upload to S3 cloud storage
6. Return AI-generated description

---

## 🧩🌐 Net+ (Advanced Networking)

Built on top of basic Socket operations, provides high-level protocol support.

### WebSocket

| Symbol | Description | Example |
|--------|-------------|---------|
| `🔌⇄` | WebSocket | `🔌⇄(url) ▷ ws` |

### Email

| Symbol | Description | Example |
|--------|-------------|---------|
| `📧` | SMTP/Email | `📧(to, body)` |

### Cloud Storage

| Symbol | Description | Example |
|--------|-------------|---------|
| `☁️` | Cloud/S3 | `file ⇢ ☁️bucket` |

### Complete Example

**Task**: WebSocket real-time notification service

```aether
🧩🌐 ⨠
🔌⇄"ws://localhost:8080" ▷ ws ⨠
↻: (
  ⏳👁"newMessage" ▷ msg ⨠
  msg ⇢ ws ⨠
  🪵"Sent: {}" ⇢ 📋msg
)
```

**Breakdown**:
1. Import Net+ module
2. Create WebSocket connection
3. Infinite loop
4. Wait for "newMessage" event
5. Send message through WebSocket
6. Log the sent message

---

## Advanced Examples

### 1. Data Analysis Pipeline (Math + Text)

**Requirements**:
- Read CSV file
- Parse rows and columns
- Calculate average of specific column
- Format and output result

**Implementation**:
```aether
🧩🧮 🧩📝 ⨠
📄📍"data.csv" ⇢ 🌊 ⇢ ✂"\n" ▷ rows ⨠
rows ⇢ ∀(r): (r ⇢ ✂"," ▷ cols ⇢ cols[2]) ▷ values ⨠
values ⇢ x̄ ▷ avg ⨠
"Average: {}" ⇢ 📋avg ⇢ 📤
```

**Traditional Python Equivalent** (~200 characters):
```python
import pandas as pd
import statistics

df = pd.read_csv("data.csv")
values = df.iloc[:, 2].tolist()
avg = statistics.mean(values)
print(f"Average: {avg}")
```

**Aether Advantage**: ~120 characters vs ~200 characters = **40% compression**

---

### 2. Smart Image Processing Service (AI + Compression + Net)

**Requirements**:
- Receive image input
- AI recognition to describe content
- Compress image
- Upload to cloud storage
- Return description

**Implementation**:
```aether
🧩🧠 🧩🗜️ 🧩🌐 ⨠
ƒ handle: 📥img ⨠
  (img ⇢ 🤖"describe") ▷ desc ⨠
  img ⇢ 📦 ▷ zip ⨠
  zip ⇢ ☁️"s3://images" ⨠
  📤desc
```

**Traditional JavaScript Equivalent** (~350 characters):
```javascript
import { recognize } from 'ai-lib';
import { compress } from 'compression';
import { S3Client } from 'aws-sdk';

async function handle(img) {
  const desc = await recognize(img, "describe");
  const zip = await compress(img);
  await S3Client.upload(zip, "s3://images");
  return desc;
}
```

**Aether Advantage**: ~150 characters vs ~350 characters = **57% compression**

---

### 3. Scheduled Database Backup (Chrono + Shell + Compression)

**Requirements**:
- Run daily at midnight
- Backup database with pg_dump
- Compress backup
- Save with date-stamped filename

**Implementation**:
```aether
🧩⏰ 🧩🗜️ ⨠
⏰"0 0 * * *": (
  🐚"pg_dump db" ⇢ 📦 ▷ backup ⨠
  backup ⇢ 📄📍(📅 ⇢ 🗓️"backup-YYYYMMDD.sql.gz")
)
```

**Traditional Bash + Python Equivalent** (~280 characters):
```bash
#!/bin/bash
# Crontab: 0 0 * * * /path/to/script.sh

from datetime import datetime
import subprocess
import gzip

date = datetime.now().strftime("backup-%Y%m%d.sql.gz")
dump = subprocess.check_output(["pg_dump", "db"])
with gzip.open(date, 'wb') as f:
    f.write(dump)
```

**Aether Advantage**: ~140 characters vs ~280 characters = **50% compression**

---

### 4. Real-Time Analytics Stream (Math + Text + Chrono)

**Requirements**:
- Process streaming data
- Calculate rolling average
- Format metrics every 5 seconds
- Output to log

**Implementation**:
```aether
🧩🧮 🧩📝 🧩⏰ ⨠
🗂[] ▷ buffer ⨠
↻: (
  📥 ⇢ J ▷ event ⨠
  buffer 🔗 event ▷ buffer ⨠
  ◇(buffer.length > 100): buffer ⇢ ⏭️1 ▷ buffer ⨠
  ⏳5s ⨠
  buffer ⇢ x̄ ▷ avg ⨠
  "Avg: {} at {}" ⇢ 📋(avg, 📅 ⇢ 🗓️"HH:mm:ss") ⇢ 🪵
)
```

**Traditional Node.js Equivalent** (~450 characters):
```javascript
const buffer = [];

setInterval(async () => {
  const event = await readInput();
  buffer.push(event);
  
  if (buffer.length > 100) {
    buffer.shift();
  }
  
  await sleep(5000);
  
  const avg = buffer.reduce((a, b) => a + b, 0) / buffer.length;
  const time = new Date().toTimeString().slice(0, 8);
  console.log(`Avg: ${avg} at ${time}`);
}, 5000);
```

**Aether Advantage**: ~250 characters vs ~450 characters = **44% compression**

---

## Standard Library Design Principles

### 1. High Leverage

The standard library focuses on operations that are:
- **Frequently needed** in real-world applications
- **Verbose** if implemented with basic symbols alone
- **Performance-critical** and benefit from native implementations

### 2. Stateless Functions

All standard library functions are **stateless** and **pure**:
- Same input always produces same output
- No side effects (except I/O operations)
- Thread-safe by design

### 3. Composability

Functions are designed for seamless composition with the `⇢` (Pipe) operator:
```aether
data ⇢ ✂"," ⇢ x̄ ⇢ 📋"Result: {}" ⇢ 📤
```

### 4. Contextual Overloading

Some symbols have different meanings in different module contexts:
- `📤`: Output (base language) vs Unpack (🧩🗜️ Compression module)
- `⚖️`: Assert (base language v1.2 testing) vs SizeOf (🧩🗜️ Compression module)

The import context determines interpretation.

### 5. Zero-Cost Abstractions

Standard library functions should compile to efficient native code:
- Direct mapping to C++/Rust implementations
- Hardware acceleration where available (SIMD, GPU)
- No runtime overhead compared to hand-written code

---

## Module Compatibility Matrix

| Module | Compatible With | Common Use Cases |
|--------|----------------|------------------|
| 🧩🧮 Math | 🧩📝 Text | Data analysis, formatting numbers |
| 🧩📝 Text | All modules | String manipulation, templates |
| 🧩⏰ Chrono | 🧩🗜️ 🧩🧠 | Scheduled tasks, time-series |
| 🧩🗜️ Compression | 🧩🌐 Net+ | File transfer, storage |
| 🧩🧠 AI | 🧩🗜️ 🧩🌐 | ML pipelines, cloud AI |
| 🧩🌐 Net+ | All modules | Distributed systems |

---

## Future Standard Library Modules

Planned modules for future versions:

### 🧩🎨 Graphics & Visualization
- Chart generation
- Image manipulation
- Color spaces

### 🧩🔊 Audio & Signal Processing
- Audio encoding/decoding
- Fourier transforms
- Signal filtering

### 🧩🗃️ Database
- SQL query builders
- NoSQL operations
- Connection pooling

### 🧩🔐 Security Advanced
- Certificate management
- OAuth flows
- Key derivation

### 🧩🌐 Blockchain
- Smart contract interactions
- Wallet operations
- Transaction signing

---

## Implementation Status

### Current Status: Documentation Phase

The ASL specification is currently in the **design and documentation phase**. This document defines:
- Module structure and semantics
- Symbol definitions and usage
- Integration patterns
- Design philosophy

### Roadmap

1. **Phase 1** (Current): Documentation and specification
2. **Phase 2**: Core module implementation in Rust
3. **Phase 3**: C++ bridge for performance-critical operations
4. **Phase 4**: Hardware acceleration (SIMD, GPU)
5. **Phase 5**: Package ecosystem and third-party modules

---

## Best Practices

### 1. Import Only What You Need

```aether
// Good: Specific imports
🧩🧮 ⨠
values ⇢ x̄ ⇢ 📤

// Avoid: Over-importing unused modules
🧩🧮 🧩📝 🧩⏰ 🧩🗜️ 🧩🧠 🧩🌐 ⨠
values ⇢ x̄ ⇢ 📤
```

### 2. Chain Operations with Pipes

```aether
// Good: Fluent pipeline
data ⇢ ✂"," ⇢ x̄ ⇢ 📋"Avg: {}" ⇢ 📤

// Avoid: Intermediate variables
data ⇢ ✂"," ▷ parts ⨠
parts ⇢ x̄ ▷ avg ⨠
avg ⇢ 📋"Avg: {}" ▷ msg ⨠
📤msg
```

### 3. Leverage Module Composition

```aether
// Good: Multi-module pipeline
🧩🧮 🧩📝 ⨠
data ⇢ x̄ ⇢ 📋"Result: {}" ⇢ 🔠 ⇢ 📤

// This combines Math (x̄) and Text (📋, 🔠) seamlessly
```

### 4. Document Complex Imports

```aether
// For complex multi-module code, add comments
🧩🧠 🧩🗜️ 🧩🌐 ⨠  // AI + Compression + Networking
ƒ process: 📥img ⨠
  img ⇢ 🤖"analyze" ⇢ 📦 ⇢ ☁️"s3://data"
```

---

## Conclusion

The **Aether Standard Library** represents a paradigm shift in how programming languages provide functionality:

- **Import as Capability**: Modules grant domain-specific powers
- **Maximum Density**: High information density through symbolic operators
- **AI-Optimized**: Designed for efficient token usage in AI models
- **Native Performance**: Direct mapping to optimized implementations
- **Composable**: Perfect integration with Aether's pipe philosophy

The ASL transforms Aether from a minimalist symbolic language into a **complete platform for modern software development**, maintaining the core vision of maximum expressiveness with minimum characters.

---

**Aether Standard Library (ASL)** - Where symbols meet capability, and brevity meets power.
