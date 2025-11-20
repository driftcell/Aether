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
            Symbol::If => "Conditional",
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
}
