# Changelog

All notable changes to Aether will be documented in this file.

## [v1.5] - HTTP Client Support

Aether v1.5 introduces **comprehensive HTTP client support** with full method coverage using reqwest and rustls for secure HTTPS.

### HTTP Methods (7 new symbol combinations)
- `🌐📥` HTTP GET - Fetch data from URL
- `🌐📤` HTTP POST - Send data to URL
- `🌐🔄` HTTP PUT - Update resource completely
- `🌐🗑️` HTTP DELETE - Remove resource
- `🌐🔧` HTTP PATCH - Partial resource update
- `🌐👁️` HTTP HEAD - Get headers only
- `🌐⚙️` HTTP OPTIONS - Get available methods

### Key Features
- ✅ Secure HTTPS with rustls (no OpenSSL dependency)
- ✅ All standard HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- ✅ Custom HTTP headers support with 🏷️ symbol
- ✅ JSON request/response support
- ✅ Automatic response parsing with status, body, headers, and JSON fields
- ✅ Async execution under the hood with tokio runtime

### Examples

**Simple API Request:**
```aether
🌐📥 "https://api.github.com/users/octocat" ▷ user ⨠ 📤 user
```

**POST with JSON:**
```aether
🌐📤 "https://httpbin.org/post" ⇢ '{"name":"Aether","version":"1.5"}' ▷ result ⨠ 📤 result
```

**Request with Custom Headers:**
```aether
// Note: Headers syntax uses 🏷️ followed by an object with header key-value pairs
// Headers must be provided as an object/map structure
🌐📥 "https://api.example.com/data" 🏷️ headers_object ▷ result
```

**Full CRUD Operations:**
```aether
// GET - Read
🌐📥 "https://api.example.com/users/1" ▷ user ⨠ 📤 user

// POST - Create
🌐📤 "https://api.example.com/users" ⇢ '{"name":"Alice"}' ▷ created ⨠ 📤 created

// PUT - Update
🌐🔄 "https://api.example.com/users/1" ⇢ '{"name":"Bob"}' ▷ updated ⨠ 📤 updated

// DELETE - Remove
🌐🗑️ "https://api.example.com/users/1" ▷ deleted ⨠ 📤 deleted
```

**Response Structure:**
```json
{
  "status": 200,
  "ok": true,
  "headers": { "content-type": "application/json", ... },
  "body": "response text...",
  "json": { ... }  // Present if body is valid JSON
}
```

## [v1.4] - Bytecode Compiler & VM

Aether v1.4 introduces bytecode compilation and a stack-based virtual machine for improved performance and distribution.

### Features
- ⚡ Compile .ae source to .aeb bytecode
- 📦 Stack-based VM execution
- 🔒 Obfuscated source code
- ✅ Pre-validated at compile time
- 🚀 Faster execution with no parsing overhead

## [v1.3] - System Programming & I/O

Aether v1.3 introduces **26 new symbols** focused on system-level programming, I/O operations, and networking.

### File System (8 symbols)
- `📄` File - File handle/object
- `📂` Dir - Directory/folder
- `📍` Path - Path resolution
- `📖` Read - Read content from file or stream
- `🖊️` Write - Write content (overwrite mode)
- `🖇️` Append - Append content (append mode)
- `🗑️` Delete - Delete file or resource
- `🛂` Perm - Permission control (chmod/chown)

### Streams & Buffers (5 symbols)
- `🌊` Stream - Data stream (Readable/Writable)
- `🧱` Buffer - Binary buffer (Bytes/Blob)
- `🌬️` Flush - Flush buffer
- `🔚` EOF - End of file/stream marker
- `⏭️` Skip - Skip bytes/move pointer

### Networking (6 symbols)
- `🔌` Socket - Network socket (TCP/UDP)
- `👂` Listen - Listen on port (Server Bind)
- `📞` Connect - Initiate connection (Client Connect)
- `🚪` Port - Port number
- `📦` Packet - Data packet (Datagram)
- `🤝` Handshake - Protocol handshake/establish connection

### Process & OS (6 symbols)
- `⚙️` Process - Process object
- `🐚` Shell - Execute shell command
- `🌍` Env - Environment variable (Get/Set)
- `🐏` Memory - Memory operations/manual allocation
- `👋` Exit - Exit program (with exit code)
- `📶` Signal - Send/capture system signal

### Examples

**High Performance Log Rotation:**
```aether
ƒ log: 📥msg ⨠ 📄📍"/var/log/app.log" ▷ f ⨠ ◇(f.size > 1GB): 🐚"mv /var/log/app.log /var/log/app.old" ⨠ msg ⇢ 🖇️f
```

**TCP Echo Server:**
```aether
🔌TCP ⨠ 👂8080 ⨠ ↻: (⏳👂 ▷ conn ⨠ ⚡(🛡(conn ⇢ 🌊 ▷ s ⨠ ↻(s ≠ 🔚): (s ⇢ 📖 ▷ data ⨠ data ⇢ 🖊️s ⨠ s ⇢ 🌬️)) ⨠ conn ⇢ 👋))
```

**Environment & Shell Integration:**
```aether
🌍"PATH" ▷ path ⨠ 🐚"ls -la" ▷ output ⨠ 📤output
```

## [v1.2] - Testing, Security & Math

Aether v1.2 introduces **15 new symbols** focused on testing, security, and scientific computing.

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

### Examples

**Secure Password Storage:**
```aether
ƒ®: 📥pass ▷ p ⨠ 🎲 ▷ salt ⨠ (p 🔗 salt) ⇢ #️⃣ ▷ hash ⨠ 💾{h:hash, s:salt} ⨠ 📤✓
```

**Unit Testing:**
```aether
🧪 "AuthTest": 🎭💾 ⨠ ⏱️("admin" ⇢ ® ▷ res) ▷ time ⨠ ⚖️(res ≡ 200) ⨠ ⚖️(time < 50ms)
```

**Vector Distance:**
```aether
ƒ calc: 📥v1 📥v2 ⨠ ((v1 - v2)↑2) ⇢ √ ▷ dist ⨠ ◇(dist ≈ 0) 📤"Same"
```

## [v1.1] - Extended Symbol System

Aether v1.1 introduces **23 new symbols** across four major categories.

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

### Example

**Concurrent Web Crawler:**
```aether
🧩🌐 ⨠ 🗂urls ▷ ∀(u): (⚡ 🛡(♻3: 🌐📥u)) ▷ res ⁇ 🛑 ⨠ ∃(res.ok) ▷ 💾
```
*(70 characters vs 300+ in traditional JavaScript - 4.3x compression!)*

## [v1.0] - Initial Release

- Core language design
- Lexer implementation
- Parser with AST generation
- Basic runtime execution
- Symbol system
- CLI interface
