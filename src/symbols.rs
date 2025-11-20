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
    /// ® - Register/create
    Register,
    
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
            Symbol::Register => "®".to_string(),
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
            Symbol::Register => "Register/create",
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
}
