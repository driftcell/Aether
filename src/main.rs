//! Aether CLI - Command line interface for the Aether programming language

use aether::{Lexer, Parser, Runtime, LANGUAGE_NAME, VERSION};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "version" | "-v" | "--version" => {
            println!("{} v{}", LANGUAGE_NAME, VERSION);
        }
        "help" | "-h" | "--help" => {
            print_help();
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage();
                process::exit(1);
            }
            let filename = &args[2];
            run_file(filename);
        }
        "symbols" => {
            print_symbols();
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: aether <command> [options]");
    println!();
    println!("Commands:");
    println!("  run <file>      Run an Aether source file");
    println!("  symbols         Display symbol reference");
    println!("  version         Display version information");
    println!("  help            Display this help message");
}

fn print_help() {
    println!("{} v{}", LANGUAGE_NAME, VERSION);
    println!();
    println!("Aether is an AI-native programming language that uses high-density");
    println!("UTF-8 symbols for maximum information compression and minimal AI token usage.");
    println!();
    print_usage();
    println!();
    println!("Examples:");
    println!("  aether run program.ae          # Run an Aether program");
    println!("  aether symbols                 # View symbol reference");
}

fn print_symbols() {
    use aether::Symbol;

    println!("{} - Symbol Reference", LANGUAGE_NAME);
    println!("{}", "=".repeat(60));
    println!();

    let symbols = vec![
        (Symbol::Function, "ƒ"),
        (Symbol::Lambda, "λ"),
        (Symbol::Input, "📥"),
        (Symbol::Output, "📤"),
        (Symbol::Persist, "💾"),
        (Symbol::Query, "🔍"),
        (Symbol::Pipe, "⇢"),
        (Symbol::PipeInto, "▷"),
        (Symbol::JsonParse, "J"),
        (Symbol::Guard, "⁇"),
        (Symbol::Halt, "🛑"),
        (Symbol::Success, "✓"),
        (Symbol::Sequence, "⨠"),
        (Symbol::If, "◇"),
        (Symbol::Or, "⊕"),
        (Symbol::And, "⊗"),
        (Symbol::Not, "¬"),
        (Symbol::Array, "🗂"),
        (Symbol::Map, "🗄"),
        (Symbol::Empty, "∅"),
        (Symbol::HttpRequest, "🌐"),
        (Symbol::Register, "®"),
    ];

    for (symbol, glyph) in symbols {
        println!("  {}  -  {}", glyph, symbol.description());
    }

    println!();
    println!("Example Program:");
    println!("  ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200");
    println!();
    println!("This represents a user registration endpoint that:");
    println!("  • Parses JSON input (📥⇢J)");
    println!("  • Pipes into variable u (▷ u)");
    println!("  • Guards against null/invalid data (⁇)");
    println!("  • Halts with 400 error if invalid (🛑400)");
    println!("  • Persists user data (💾u)");
    println!("  • Returns 200 success (📤200)");
}

fn run_file(filename: &str) {
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    println!("Running Aether program: {}", filename);
    println!("{}", "-".repeat(60));

    // Lexer
    let mut lexer = Lexer::new(source.clone());
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            process::exit(1);
        }
    };

    println!("Tokens: {} token(s) generated", tokens.len());

    // Parser
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(err) => {
            eprintln!("Parser error: {}", err);
            process::exit(1);
        }
    };

    println!("AST: {} node(s) parsed", ast.len());

    // Runtime
    let mut runtime = Runtime::new();
    match runtime.execute(ast) {
        Ok(result) => {
            println!("{}", "-".repeat(60));
            println!("Execution completed successfully");
            println!("Result: {:?}", result);
        }
        Err(err) => {
            eprintln!("{}", "-".repeat(60));
            eprintln!("Runtime error: {}", err);
            process::exit(1);
        }
    }
}
