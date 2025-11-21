//! Symbol definitions for Aether language
//!
//! Aether uses UTF-8 symbols for high-density code representation.
//! Each symbol represents a fundamental operation or concept.

/// Core symbols in the Aether language
#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    // Function and control flow
    /// ƒ - Function definition
    Function,
    /// λ - Lambda/anonymous function
    Lambda,
    /// ⇒ - Map/transform
    MapArrow,
    /// ⇢ - Pipe/flow
    Pipe,
    /// ▷ - Pipe into variable
    PipeInto,
    
    // Data operations
    /// 📥 - Input/Request context
    Input,
    /// 📤 - Output/Response
    Output,
    /// 💾 - Persist/Database operation
    Persist,
    /// 🔍 - Query/Search
    Query,
    
    // JSON and data parsing
    /// J - JSON parse
    JsonParse,
    /// S - String
    StringType,
    /// N - Number
    NumberType,
    
    // Logic and control
    /// ⁇ - Guard/null check
    Guard,
    /// 🛑 - Halt/error
    Halt,
    /// ✓ - Success/validate
    Success,
    /// ⨠ - Sequence/then
    Sequence,
    
    // Conditionals
    /// ◇ - If/conditional
    If,
    /// ◈ - ElseIf/conditional alternative
    ElseIf,
    /// ◆ - Else/default branch
    Else,
    /// ⊕ - Or
    Or,
    /// ⊗ - And
    And,
    /// ¬ - Not
    Not,
    
    // Collections
    /// 🗂 - Array/List
    Array,
    /// 🗄 - Map/Dictionary
    Map,
    /// ∅ - Empty/null
    Empty,
    
    // Network operations
    /// 🌐 - HTTP request
    HttpRequest,
    /// 🌐📥 - HTTP GET (composed)
    HttpGet,
    /// ® - Register/create
    Register,
    
    // Control Flow & Iteration (v1.1)
    /// ↻ - Loop/While
    Loop,
    /// ∀ - ForEach/Map
    ForEach,
    /// ∃ - Filter/Find
    Filter,
    /// ∑ - Reduce/Sum
    Reduce,
    /// 🛡 - Try/Rescue
    Try,
    /// ♻ - Retry
    Retry,
    
    // Concurrency & Async (v1.1)
    /// ⚡ - Async/Trigger
    Async,
    /// ⏳ - Await
    Await,
    /// 🧵 - Thread/Task
    Thread,
    /// 🔒 - Mutex/Lock
    Lock,
    /// 📡 - Emit/Signal
    Emit,
    /// 👁 - Watch/Listen
    Watch,
    
    // Data Manipulation (v1.1)
    /// ✂ - Split/Slice
    Split,
    /// 🔗 - Join/Concat
    Join,
    /// ✱ - Regex/Match
    Regex,
    /// ≡ - Equal
    Equal,
    /// ≠ - Not Equal
    NotEqual,
    /// 🧊 - Immutable/Const
    Immutable,
    
    // System & Environment (v1.1)
    /// 🧩 - Import/Module
    Import,
    /// 🔑 - Auth/Token
    Auth,
    /// 📅 - Date/Time
    DateTime,
    /// 🎲 - Random
    Random,
    /// 🪵 - Log
    Log,
    
    // Testing & Debugging (v1.2)
    /// 🧪 - Test/Suite
    Test,
    /// ⚖️ - Assert
    Assert,
    /// 🎭 - Mock/Stub
    Mock,
    /// ⏱️ - Benchmark
    Benchmark,
    /// 🐛 - Debug
    Debug,
    
    // Security & Crypto (v1.2)
    /// 🔐 - Encrypt
    Encrypt,
    /// 🔓 - Decrypt
    Decrypt,
    /// #️⃣ - Hash
    Hash,
    /// ✍️ - Sign
    Sign,
    /// 🛡️ - Verify (different from 🛡 Try)
    Verify,
    
    // Math & Science (v1.2)
    /// ↑ - Power
    Power,
    /// √ - Root
    Root,
    /// ≈ - Approx
    Approx,
    /// ∞ - Infinity
    Infinity,
    /// ∆ - Delta
    Delta,
    
    // File System (v1.3)
    /// 📄 - File handle/object
    File,
    /// 📂 - Directory/folder
    Dir,
    /// 📍 - Path resolution
    Path,
    /// 📖 - Read content from file or stream
    Read,
    /// 🖊️ - Write content (overwrite mode)
    Write,
    /// 🖇️ - Append content (append mode)
    Append,
    /// 🗑️ - Delete file or resource
    Delete,
    /// 🛂 - Permission control (chmod/chown)
    Perm,
    
    // Streams & Buffers (v1.3)
    /// 🌊 - Data stream (Readable/Writable Stream)
    Stream,
    /// 🧱 - Binary buffer (Bytes/Blob)
    Buffer,
    /// 🌬️ - Flush buffer
    Flush,
    /// 🔚 - End of file/stream marker
    Eof,
    /// ⏭️ - Skip bytes/move pointer
    Skip,
    
    // Networking (v1.3)
    /// 🔌 - Network socket (TCP/UDP)
    Socket,
    /// 👂 - Listen on port (Server Bind)
    Listen,
    /// 📞 - Initiate connection (Client Connect)
    Connect,
    /// 🚪 - Port number
    Port,
    /// 📦 - Data packet (Datagram)
    Packet,
    /// 🤝 - Protocol handshake/establish connection
    Handshake,
    
    // Process & OS (v1.3)
    /// ⚙️ - Process object
    Process,
    /// 🐚 - Execute shell command
    Shell,
    /// 🌍 - Environment variable (Get/Set)
    Env,
    /// 🐏 - Memory operations/manual allocation
    Memory,
    /// 👋 - Exit program (with exit code)
    Exit,
    /// 📶 - Send/capture system signal
    Signal,
    
    // Variables and identifiers
    /// Generic identifier
    Identifier(String),
    
    // Literals
    /// String literal
    StringLiteral(String),
    /// Number literal
    NumberLiteral(f64),
    
    // Special
    /// End of statement
    Eos,
    /// Separator
    Separator,
}

impl Symbol {
    /// Convert a character/string to a Symbol
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ƒ" => Some(Symbol::Function),
            "λ" => Some(Symbol::Lambda),
            "⇒" => Some(Symbol::MapArrow),
            "⇢" => Some(Symbol::Pipe),
            "▷" => Some(Symbol::PipeInto),
            "📥" => Some(Symbol::Input),
            "📤" => Some(Symbol::Output),
            "💾" => Some(Symbol::Persist),
            "🔍" => Some(Symbol::Query),
            "J" => Some(Symbol::JsonParse),
            "S" => Some(Symbol::StringType),
            "N" => Some(Symbol::NumberType),
            "⁇" => Some(Symbol::Guard),
            "🛑" => Some(Symbol::Halt),
            "✓" => Some(Symbol::Success),
            "⨠" => Some(Symbol::Sequence),
            "◇" => Some(Symbol::If),
            "◈" => Some(Symbol::ElseIf),
            "◆" => Some(Symbol::Else),
            "⊕" => Some(Symbol::Or),
            "⊗" => Some(Symbol::And),
            "¬" => Some(Symbol::Not),
            "🗂" => Some(Symbol::Array),
            "🗄" => Some(Symbol::Map),
            "∅" => Some(Symbol::Empty),
            "🌐" => Some(Symbol::HttpRequest),
            "®" => Some(Symbol::Register),
            // Control Flow & Iteration
            "↻" => Some(Symbol::Loop),
            "∀" => Some(Symbol::ForEach),
            "∃" => Some(Symbol::Filter),
            "∑" => Some(Symbol::Reduce),
            "🛡" => Some(Symbol::Try),
            "♻" => Some(Symbol::Retry),
            // Concurrency & Async
            "⚡" => Some(Symbol::Async),
            "⏳" => Some(Symbol::Await),
            "🧵" => Some(Symbol::Thread),
            "🔒" => Some(Symbol::Lock),
            "📡" => Some(Symbol::Emit),
            "👁" => Some(Symbol::Watch),
            // Data Manipulation
            "✂" => Some(Symbol::Split),
            "🔗" => Some(Symbol::Join),
            "✱" => Some(Symbol::Regex),
            "≡" => Some(Symbol::Equal),
            "≠" => Some(Symbol::NotEqual),
            "🧊" => Some(Symbol::Immutable),
            // System & Environment
            "🧩" => Some(Symbol::Import),
            "🔑" => Some(Symbol::Auth),
            "📅" => Some(Symbol::DateTime),
            "🎲" => Some(Symbol::Random),
            "🪵" => Some(Symbol::Log),
            // Testing & Debugging
            "🧪" => Some(Symbol::Test),
            "⚖️" => Some(Symbol::Assert),
            "🎭" => Some(Symbol::Mock),
            "⏱️" => Some(Symbol::Benchmark),
            "🐛" => Some(Symbol::Debug),
            // Security & Crypto
            "🔐" => Some(Symbol::Encrypt),
            "🔓" => Some(Symbol::Decrypt),
            "#️⃣" => Some(Symbol::Hash),
            "✍️" => Some(Symbol::Sign),
            "🛡️" => Some(Symbol::Verify),
            // Math & Science
            "↑" => Some(Symbol::Power),
            "√" => Some(Symbol::Root),
            "≈" => Some(Symbol::Approx),
            "∞" => Some(Symbol::Infinity),
            "∆" => Some(Symbol::Delta),
            // File System
            "📄" => Some(Symbol::File),
            "📂" => Some(Symbol::Dir),
            "📍" => Some(Symbol::Path),
            "📖" => Some(Symbol::Read),
            "🖊️" => Some(Symbol::Write),
            "🖇️" => Some(Symbol::Append),
            "🗑️" => Some(Symbol::Delete),
            "🛂" => Some(Symbol::Perm),
            // Streams & Buffers
            "🌊" => Some(Symbol::Stream),
            "🧱" => Some(Symbol::Buffer),
            "🌬️" => Some(Symbol::Flush),
            "🔚" => Some(Symbol::Eof),
            "⏭️" => Some(Symbol::Skip),
            // Networking
            "🔌" => Some(Symbol::Socket),
            "👂" => Some(Symbol::Listen),
            "📞" => Some(Symbol::Connect),
            "🚪" => Some(Symbol::Port),
            "📦" => Some(Symbol::Packet),
            "🤝" => Some(Symbol::Handshake),
            // Process & OS
            "⚙️" => Some(Symbol::Process),
            "🐚" => Some(Symbol::Shell),
            "🌍" => Some(Symbol::Env),
            "🐏" => Some(Symbol::Memory),
            "👋" => Some(Symbol::Exit),
            "📶" => Some(Symbol::Signal),
            _ => None,
        }
    }

    /// Convert Symbol back to its UTF-8 representation
    pub fn to_str(&self) -> String {
        match self {
            Symbol::Function => "ƒ".to_string(),
            Symbol::Lambda => "λ".to_string(),
            Symbol::MapArrow => "⇒".to_string(),
            Symbol::Pipe => "⇢".to_string(),
            Symbol::PipeInto => "▷".to_string(),
            Symbol::Input => "📥".to_string(),
            Symbol::Output => "📤".to_string(),
            Symbol::Persist => "💾".to_string(),
            Symbol::Query => "🔍".to_string(),
            Symbol::JsonParse => "J".to_string(),
            Symbol::StringType => "S".to_string(),
            Symbol::NumberType => "N".to_string(),
            Symbol::Guard => "⁇".to_string(),
            Symbol::Halt => "🛑".to_string(),
            Symbol::Success => "✓".to_string(),
            Symbol::Sequence => "⨠".to_string(),
            Symbol::If => "◇".to_string(),
            Symbol::ElseIf => "◈".to_string(),
            Symbol::Else => "◆".to_string(),
            Symbol::Or => "⊕".to_string(),
            Symbol::And => "⊗".to_string(),
            Symbol::Not => "¬".to_string(),
            Symbol::Array => "🗂".to_string(),
            Symbol::Map => "🗄".to_string(),
            Symbol::Empty => "∅".to_string(),
            Symbol::HttpRequest => "🌐".to_string(),
            Symbol::HttpGet => "🌐📥".to_string(),
            Symbol::Register => "®".to_string(),
            // Control Flow & Iteration
            Symbol::Loop => "↻".to_string(),
            Symbol::ForEach => "∀".to_string(),
            Symbol::Filter => "∃".to_string(),
            Symbol::Reduce => "∑".to_string(),
            Symbol::Try => "🛡".to_string(),
            Symbol::Retry => "♻".to_string(),
            // Concurrency & Async
            Symbol::Async => "⚡".to_string(),
            Symbol::Await => "⏳".to_string(),
            Symbol::Thread => "🧵".to_string(),
            Symbol::Lock => "🔒".to_string(),
            Symbol::Emit => "📡".to_string(),
            Symbol::Watch => "👁".to_string(),
            // Data Manipulation
            Symbol::Split => "✂".to_string(),
            Symbol::Join => "🔗".to_string(),
            Symbol::Regex => "✱".to_string(),
            Symbol::Equal => "≡".to_string(),
            Symbol::NotEqual => "≠".to_string(),
            Symbol::Immutable => "🧊".to_string(),
            // System & Environment
            Symbol::Import => "🧩".to_string(),
            Symbol::Auth => "🔑".to_string(),
            Symbol::DateTime => "📅".to_string(),
            Symbol::Random => "🎲".to_string(),
            Symbol::Log => "🪵".to_string(),
            // Testing & Debugging
            Symbol::Test => "🧪".to_string(),
            Symbol::Assert => "⚖️".to_string(),
            Symbol::Mock => "🎭".to_string(),
            Symbol::Benchmark => "⏱️".to_string(),
            Symbol::Debug => "🐛".to_string(),
            // Security & Crypto
            Symbol::Encrypt => "🔐".to_string(),
            Symbol::Decrypt => "🔓".to_string(),
            Symbol::Hash => "#️⃣".to_string(),
            Symbol::Sign => "✍️".to_string(),
            Symbol::Verify => "🛡️".to_string(),
            // Math & Science
            Symbol::Power => "↑".to_string(),
            Symbol::Root => "√".to_string(),
            Symbol::Approx => "≈".to_string(),
            Symbol::Infinity => "∞".to_string(),
            Symbol::Delta => "∆".to_string(),
            // File System
            Symbol::File => "📄".to_string(),
            Symbol::Dir => "📂".to_string(),
            Symbol::Path => "📍".to_string(),
            Symbol::Read => "📖".to_string(),
            Symbol::Write => "🖊️".to_string(),
            Symbol::Append => "🖇️".to_string(),
            Symbol::Delete => "🗑️".to_string(),
            Symbol::Perm => "🛂".to_string(),
            // Streams & Buffers
            Symbol::Stream => "🌊".to_string(),
            Symbol::Buffer => "🧱".to_string(),
            Symbol::Flush => "🌬️".to_string(),
            Symbol::Eof => "🔚".to_string(),
            Symbol::Skip => "⏭️".to_string(),
            // Networking
            Symbol::Socket => "🔌".to_string(),
            Symbol::Listen => "👂".to_string(),
            Symbol::Connect => "📞".to_string(),
            Symbol::Port => "🚪".to_string(),
            Symbol::Packet => "📦".to_string(),
            Symbol::Handshake => "🤝".to_string(),
            // Process & OS
            Symbol::Process => "⚙️".to_string(),
            Symbol::Shell => "🐚".to_string(),
            Symbol::Env => "🌍".to_string(),
            Symbol::Memory => "🐏".to_string(),
            Symbol::Exit => "👋".to_string(),
            Symbol::Signal => "📶".to_string(),
            Symbol::Identifier(id) => id.clone(),
            Symbol::StringLiteral(s) => format!("\"{}\"", s),
            Symbol::NumberLiteral(n) => n.to_string(),
            Symbol::Eos => ";".to_string(),
            Symbol::Separator => ",".to_string(),
        }
    }

    /// Get a human-readable description of the symbol
    pub fn description(&self) -> &str {
        match self {
            Symbol::Function => "Function definition",
            Symbol::Lambda => "Lambda/anonymous function",
            Symbol::MapArrow => "Map/transform operation",
            Symbol::Pipe => "Pipe/flow data",
            Symbol::PipeInto => "Pipe into variable",
            Symbol::Input => "Input/Request context",
            Symbol::Output => "Output/Response",
            Symbol::Persist => "Persist to database",
            Symbol::Query => "Query/Search operation",
            Symbol::JsonParse => "Parse JSON",
            Symbol::StringType => "String type",
            Symbol::NumberType => "Number type",
            Symbol::Guard => "Guard/null check",
            Symbol::Halt => "Halt/terminate with error",
            Symbol::Success => "Success/validate",
            Symbol::Sequence => "Sequence operations",
            Symbol::If => "Conditional (if)",
            Symbol::ElseIf => "Conditional alternative (else if)",
            Symbol::Else => "Default branch (else)",
            Symbol::Or => "Logical OR",
            Symbol::And => "Logical AND",
            Symbol::Not => "Logical NOT",
            Symbol::Array => "Array/List",
            Symbol::Map => "Map/Dictionary",
            Symbol::Empty => "Empty/null value",
            Symbol::HttpRequest => "HTTP request",
            Symbol::HttpGet => "HTTP GET request",
            Symbol::Register => "Register/create",
            // Control Flow & Iteration
            Symbol::Loop => "Loop/While (unbounded loop)",
            Symbol::ForEach => "ForEach/Map over collection",
            Symbol::Filter => "Filter/Find in collection",
            Symbol::Reduce => "Reduce/Sum aggregation",
            Symbol::Try => "Try/Rescue exception handling",
            Symbol::Retry => "Retry on failure",
            // Concurrency & Async
            Symbol::Async => "Async execution/Trigger",
            Symbol::Await => "Await async result",
            Symbol::Thread => "Thread/Task concurrent execution",
            Symbol::Lock => "Mutex/Lock critical section",
            Symbol::Emit => "Emit/Signal event",
            Symbol::Watch => "Watch/Listen to events",
            // Data Manipulation
            Symbol::Split => "Split/Slice string or array",
            Symbol::Join => "Join/Concat elements",
            Symbol::Regex => "Regex/Pattern match",
            Symbol::Equal => "Strict equality comparison",
            Symbol::NotEqual => "Not equal comparison",
            Symbol::Immutable => "Immutable/Const definition",
            // System & Environment
            Symbol::Import => "Import module/dependency",
            Symbol::Auth => "Authentication/Token",
            Symbol::DateTime => "Date/Time operations",
            Symbol::Random => "Random number generation",
            Symbol::Log => "Log message",
            // Testing & Debugging
            Symbol::Test => "Test case/suite definition",
            Symbol::Assert => "Assert condition (fail if false)",
            Symbol::Mock => "Mock external dependencies",
            Symbol::Benchmark => "Measure execution time",
            Symbol::Debug => "Debug mode/breakpoint",
            // Security & Crypto
            Symbol::Encrypt => "Encrypt data",
            Symbol::Decrypt => "Decrypt data",
            Symbol::Hash => "Calculate hash value",
            Symbol::Sign => "Digital signature",
            Symbol::Verify => "Verify signature",
            // Math & Science
            Symbol::Power => "Power operation (exponentiation)",
            Symbol::Root => "Square root",
            Symbol::Approx => "Approximate equality",
            Symbol::Infinity => "Infinity value",
            Symbol::Delta => "Change/difference value",
            // File System
            Symbol::File => "File handle/object",
            Symbol::Dir => "Directory/folder",
            Symbol::Path => "Path resolution",
            Symbol::Read => "Read content from file or stream",
            Symbol::Write => "Write content (overwrite mode)",
            Symbol::Append => "Append content (append mode)",
            Symbol::Delete => "Delete file or resource",
            Symbol::Perm => "Permission control (chmod/chown)",
            // Streams & Buffers
            Symbol::Stream => "Data stream (Readable/Writable)",
            Symbol::Buffer => "Binary buffer (Bytes/Blob)",
            Symbol::Flush => "Flush buffer",
            Symbol::Eof => "End of file/stream marker",
            Symbol::Skip => "Skip bytes/move pointer",
            // Networking
            Symbol::Socket => "Network socket (TCP/UDP)",
            Symbol::Listen => "Listen on port (Server Bind)",
            Symbol::Connect => "Initiate connection (Client Connect)",
            Symbol::Port => "Port number",
            Symbol::Packet => "Data packet (Datagram)",
            Symbol::Handshake => "Protocol handshake/establish connection",
            // Process & OS
            Symbol::Process => "Process object",
            Symbol::Shell => "Execute shell command",
            Symbol::Env => "Environment variable (Get/Set)",
            Symbol::Memory => "Memory operations/manual allocation",
            Symbol::Exit => "Exit program (with exit code)",
            Symbol::Signal => "Send/capture system signal",
            Symbol::Identifier(_) => "Identifier",
            Symbol::StringLiteral(_) => "String literal",
            Symbol::NumberLiteral(_) => "Number literal",
            Symbol::Eos => "End of statement",
            Symbol::Separator => "Separator",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_roundtrip() {
        let symbols = vec![
            Symbol::Function,
            Symbol::Input,
            Symbol::Output,
            Symbol::Pipe,
            Symbol::Guard,
        ];

        for symbol in symbols {
            let str_repr = symbol.to_str();
            let parsed = Symbol::from_str(&str_repr);
            assert_eq!(Some(symbol), parsed);
        }
    }

    #[test]
    fn test_symbol_descriptions() {
        assert_eq!(Symbol::Function.description(), "Function definition");
        assert_eq!(Symbol::Guard.description(), "Guard/null check");
    }
    
    #[test]
    fn test_control_flow_symbols() {
        let symbols = vec![
            (Symbol::Loop, "↻"),
            (Symbol::ForEach, "∀"),
            (Symbol::Filter, "∃"),
            (Symbol::Reduce, "∑"),
            (Symbol::Try, "🛡"),
            (Symbol::Retry, "♻"),
        ];
        
        for (symbol, expected_str) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol));
        }
    }
    
    #[test]
    fn test_async_symbols() {
        let symbols = vec![
            (Symbol::Async, "⚡"),
            (Symbol::Await, "⏳"),
            (Symbol::Thread, "🧵"),
            (Symbol::Lock, "🔒"),
            (Symbol::Emit, "📡"),
            (Symbol::Watch, "👁"),
        ];
        
        for (symbol, expected_str) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol));
        }
    }
    
    #[test]
    fn test_data_manipulation_symbols() {
        let symbols = vec![
            (Symbol::Split, "✂"),
            (Symbol::Join, "🔗"),
            (Symbol::Regex, "✱"),
            (Symbol::Equal, "≡"),
            (Symbol::NotEqual, "≠"),
            (Symbol::Immutable, "🧊"),
        ];
        
        for (symbol, expected_str) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol));
        }
    }
    
    #[test]
    fn test_system_symbols() {
        let symbols = vec![
            (Symbol::Import, "🧩"),
            (Symbol::Auth, "🔑"),
            (Symbol::DateTime, "📅"),
            (Symbol::Random, "🎲"),
            (Symbol::Log, "🪵"),
        ];
        
        for (symbol, expected_str) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol));
        }
    }
    
    #[test]
    fn test_new_symbol_descriptions() {
        assert_eq!(Symbol::ForEach.description(), "ForEach/Map over collection");
        assert_eq!(Symbol::Async.description(), "Async execution/Trigger");
        assert_eq!(Symbol::Split.description(), "Split/Slice string or array");
        assert_eq!(Symbol::Import.description(), "Import module/dependency");
    }
    
    #[test]
    fn test_testing_debugging_symbols() {
        let symbols = vec![
            (Symbol::Test, "🧪", "Test case/suite definition"),
            (Symbol::Assert, "⚖️", "Assert condition (fail if false)"),
            (Symbol::Mock, "🎭", "Mock external dependencies"),
            (Symbol::Benchmark, "⏱️", "Measure execution time"),
            (Symbol::Debug, "🐛", "Debug mode/breakpoint"),
        ];
        
        for (symbol, expected_str, expected_desc) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol.clone()));
            assert_eq!(symbol.description(), expected_desc);
        }
    }
    
    #[test]
    fn test_security_crypto_symbols() {
        let symbols = vec![
            (Symbol::Encrypt, "🔐", "Encrypt data"),
            (Symbol::Decrypt, "🔓", "Decrypt data"),
            (Symbol::Hash, "#️⃣", "Calculate hash value"),
            (Symbol::Sign, "✍️", "Digital signature"),
            (Symbol::Verify, "🛡️", "Verify signature"),
        ];
        
        for (symbol, expected_str, expected_desc) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol.clone()));
            assert_eq!(symbol.description(), expected_desc);
        }
    }
    
    #[test]
    fn test_math_science_symbols() {
        let symbols = vec![
            (Symbol::Power, "↑", "Power operation (exponentiation)"),
            (Symbol::Root, "√", "Square root"),
            (Symbol::Approx, "≈", "Approximate equality"),
            (Symbol::Infinity, "∞", "Infinity value"),
            (Symbol::Delta, "∆", "Change/difference value"),
        ];
        
        for (symbol, expected_str, expected_desc) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol.clone()));
            assert_eq!(symbol.description(), expected_desc);
        }
    }
    
    #[test]
    fn test_verify_vs_try_distinction() {
        // Ensure 🛡️ (Verify) is different from 🛡 (Try)
        assert_ne!(Symbol::Verify.to_str(), Symbol::Try.to_str());
        assert_eq!(Symbol::Verify.to_str(), "🛡️");
        assert_eq!(Symbol::Try.to_str(), "🛡");
        
        // Test parsing
        assert_eq!(Symbol::from_str("🛡️"), Some(Symbol::Verify));
        assert_eq!(Symbol::from_str("🛡"), Some(Symbol::Try));
    }
    
    #[test]
    fn test_file_system_symbols() {
        let symbols = vec![
            (Symbol::File, "📄", "File handle/object"),
            (Symbol::Dir, "📂", "Directory/folder"),
            (Symbol::Path, "📍", "Path resolution"),
            (Symbol::Read, "📖", "Read content from file or stream"),
            (Symbol::Write, "🖊️", "Write content (overwrite mode)"),
            (Symbol::Append, "🖇️", "Append content (append mode)"),
            (Symbol::Delete, "🗑️", "Delete file or resource"),
            (Symbol::Perm, "🛂", "Permission control (chmod/chown)"),
        ];
        
        for (symbol, expected_str, expected_desc) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol.clone()));
            assert_eq!(symbol.description(), expected_desc);
        }
    }
    
    #[test]
    fn test_streams_buffers_symbols() {
        let symbols = vec![
            (Symbol::Stream, "🌊", "Data stream (Readable/Writable)"),
            (Symbol::Buffer, "🧱", "Binary buffer (Bytes/Blob)"),
            (Symbol::Flush, "🌬️", "Flush buffer"),
            (Symbol::Eof, "🔚", "End of file/stream marker"),
            (Symbol::Skip, "⏭️", "Skip bytes/move pointer"),
        ];
        
        for (symbol, expected_str, expected_desc) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol.clone()));
            assert_eq!(symbol.description(), expected_desc);
        }
    }
    
    #[test]
    fn test_networking_symbols() {
        let symbols = vec![
            (Symbol::Socket, "🔌", "Network socket (TCP/UDP)"),
            (Symbol::Listen, "👂", "Listen on port (Server Bind)"),
            (Symbol::Connect, "📞", "Initiate connection (Client Connect)"),
            (Symbol::Port, "🚪", "Port number"),
            (Symbol::Packet, "📦", "Data packet (Datagram)"),
            (Symbol::Handshake, "🤝", "Protocol handshake/establish connection"),
        ];
        
        for (symbol, expected_str, expected_desc) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol.clone()));
            assert_eq!(symbol.description(), expected_desc);
        }
    }
    
    #[test]
    fn test_process_os_symbols() {
        let symbols = vec![
            (Symbol::Process, "⚙️", "Process object"),
            (Symbol::Shell, "🐚", "Execute shell command"),
            (Symbol::Env, "🌍", "Environment variable (Get/Set)"),
            (Symbol::Memory, "🐏", "Memory operations/manual allocation"),
            (Symbol::Exit, "👋", "Exit program (with exit code)"),
            (Symbol::Signal, "📶", "Send/capture system signal"),
        ];
        
        for (symbol, expected_str, expected_desc) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol.clone()));
            assert_eq!(symbol.description(), expected_desc);
        }
    }
    
    #[test]
    fn test_conditional_symbols() {
        let symbols = vec![
            (Symbol::If, "◇", "Conditional (if)"),
            (Symbol::ElseIf, "◈", "Conditional alternative (else if)"),
            (Symbol::Else, "◆", "Default branch (else)"),
            (Symbol::Or, "⊕", "Logical OR"),
            (Symbol::And, "⊗", "Logical AND"),
            (Symbol::Not, "¬", "Logical NOT"),
        ];
        
        for (symbol, expected_str, expected_desc) in symbols {
            assert_eq!(symbol.to_str(), expected_str);
            assert_eq!(Symbol::from_str(expected_str), Some(symbol.clone()));
            assert_eq!(symbol.description(), expected_desc);
        }
    }
}
