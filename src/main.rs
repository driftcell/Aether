//! Aether CLI - Command line interface for the Aether programming language

use aether::{Lexer, Parser, Runtime, Compiler, VM, BytecodeProgram, LANGUAGE_NAME, VERSION};
use std::env;
use std::fs;
use std::io::{BufReader, BufWriter};
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
        "compile" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage();
                process::exit(1);
            }
            let input_file = &args[2];
            let output_file = if args.len() >= 4 {
                args[3].clone()
            } else {
                // Replace .ae extension with .aeb
                input_file.replace(".ae", ".aeb")
            };
            compile_file(input_file, &output_file);
        }
        "exec" => {
            if args.len() < 3 {
                eprintln!("Error: No bytecode file specified");
                print_usage();
                process::exit(1);
            }
            let filename = &args[2];
            exec_bytecode(filename);
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
    println!("  run <file>              Run an Aether source file (.ae)");
    println!("  compile <file> [out]    Compile .ae source to .aeb bytecode");
    println!("  exec <file>             Execute .aeb bytecode file");
    println!("  symbols                 Display symbol reference");
    println!("  version                 Display version information");
    println!("  help                    Display this help message");
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
    println!("  aether run program.ae              # Run an Aether program");
    println!("  aether compile program.ae          # Compile to program.aeb");
    println!("  aether exec program.aeb            # Execute bytecode");
    println!("  aether symbols                     # View symbol reference");
}

fn print_symbols() {
    use aether::Symbol;

    println!("{} - Symbol Reference", LANGUAGE_NAME);
    println!("{}", "=".repeat(60));
    println!();

    let symbols = vec![
        // Core symbols
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
    println!("--- v1.1 Symbols ---");
    println!();
    
    let v11_symbols = vec![
        // Control Flow & Iteration
        (Symbol::Loop, "↻"),
        (Symbol::ForEach, "∀"),
        (Symbol::Filter, "∃"),
        (Symbol::Reduce, "∑"),
        (Symbol::Try, "🛡"),
        (Symbol::Retry, "♻"),
        // Concurrency & Async
        (Symbol::Async, "⚡"),
        (Symbol::Await, "⏳"),
        (Symbol::Thread, "🧵"),
        (Symbol::Lock, "🔒"),
        (Symbol::Emit, "📡"),
        (Symbol::Watch, "👁"),
        // Data Manipulation
        (Symbol::Split, "✂"),
        (Symbol::Join, "🔗"),
        (Symbol::Regex, "✱"),
        (Symbol::Equal, "≡"),
        (Symbol::NotEqual, "≠"),
        (Symbol::Immutable, "🧊"),
        // System & Environment
        (Symbol::Import, "🧩"),
        (Symbol::Auth, "🔑"),
        (Symbol::DateTime, "📅"),
        (Symbol::Random, "🎲"),
        (Symbol::Log, "🪵"),
    ];

    for (symbol, glyph) in v11_symbols {
        println!("  {}  -  {}", glyph, symbol.description());
    }

    println!();
    println!("--- v1.2 Symbols ---");
    println!();
    
    let v12_symbols = vec![
        // Testing & Debugging
        (Symbol::Test, "🧪"),
        (Symbol::Assert, "⚖️"),
        (Symbol::Mock, "🎭"),
        (Symbol::Benchmark, "⏱️"),
        (Symbol::Debug, "🐛"),
        // Security & Crypto
        (Symbol::Encrypt, "🔐"),
        (Symbol::Decrypt, "🔓"),
        (Symbol::Hash, "#️⃣"),
        (Symbol::Sign, "✍️"),
        (Symbol::Verify, "🛡️"),
        // Math & Science
        (Symbol::Power, "↑"),
        (Symbol::Root, "√"),
        (Symbol::Approx, "≈"),
        (Symbol::Infinity, "∞"),
        (Symbol::Delta, "∆"),
    ];

    for (symbol, glyph) in v12_symbols {
        println!("  {}  -  {}", glyph, symbol.description());
    }

    println!();
    println!("--- v1.3 Symbols ---");
    println!();
    
    let v13_symbols = vec![
        // File System
        (Symbol::File, "📄"),
        (Symbol::Dir, "📂"),
        (Symbol::Path, "📍"),
        (Symbol::Read, "📖"),
        (Symbol::Write, "🖊️"),
        (Symbol::Append, "🖇️"),
        (Symbol::Delete, "🗑️"),
        (Symbol::Perm, "🛂"),
        // Streams & Buffers
        (Symbol::Stream, "🌊"),
        (Symbol::Buffer, "🧱"),
        (Symbol::Flush, "🌬️"),
        (Symbol::Eof, "🔚"),
        (Symbol::Skip, "⏭️"),
        // Networking
        (Symbol::Socket, "🔌"),
        (Symbol::Listen, "👂"),
        (Symbol::Connect, "📞"),
        (Symbol::Port, "🚪"),
        (Symbol::Packet, "📦"),
        (Symbol::Handshake, "🤝"),
        // Process & OS
        (Symbol::Process, "⚙️"),
        (Symbol::Shell, "🐚"),
        (Symbol::Env, "🌍"),
        (Symbol::Memory, "🐏"),
        (Symbol::Exit, "👋"),
        (Symbol::Signal, "📶"),
    ];

    for (symbol, glyph) in v13_symbols {
        println!("  {}  -  {}", glyph, symbol.description());
    }

    println!();
    println!("Example Programs:");
    println!();
    println!("User Registration:");
    println!("  ƒ®: 📥⇢J ▷ u ⁇ 🛑400 ⨠ 💾u ⨠ 📤200");
    println!();
    println!("Secure Password Storage (v1.2):");
    println!("  ƒ®: 📥pass ▷ p ⨠ 🎲 ▷ salt ⨠ (p 🔗 salt) ⇢ #️⃣ ▷ hash ⨠ 💾{{h:hash, s:salt}} ⨠ 📤✓");
    println!();
    println!("Unit Test (v1.2):");
    println!("  🧪 \"AuthTest\": 🎭💾 ⨠ ⏱️(\"admin\" ⇢ ® ▷ res) ▷ time ⨠ ⚖️(res ≡ 200) ⨠ ⚖️(time < 50ms)");
    println!();
    println!("Log Rotation (v1.3):");
    println!("  ƒ log: 📥msg ⨠ 📄📍\"/var/log/app.log\" ▷ f ⨠ ◇(f.size > 1GB): 🐚\"mv /var/log/app.log /var/log/app.old\" ⨠ msg ⇢ 🖇️f");
    println!();
    println!("TCP Echo Server (v1.3):");
    println!("  🔌TCP ⨠ 👂8080 ⨠ ↻: (⏳👂 ▷ conn ⨠ ⚡(🛡(conn ⇢ 🌊 ▷ s ⨠ ↻(s ≠ 🔚): (s ⇢ 📖 ▷ data ⨠ data ⇢ 🖊️s ⨠ s ⇢ 🌬️)) ⨠ conn ⇢ 👋))");
    println!();
    println!("Environment Variable & Shell (v1.3):");
    println!("  🌍\"PATH\" ▷ path ⨠ 🐚\"ls -la\" ▷ output ⨠ 📤output");
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

fn compile_file(input_file: &str, output_file: &str) {
    println!("Compiling {} to {}...", input_file, output_file);
    println!("{}", "-".repeat(60));

    // Read source file
    let source = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", input_file, err);
            process::exit(1);
        }
    };

    // Lexer
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            process::exit(1);
        }
    };
    println!("✓ Lexer: {} tokens generated", tokens.len());

    // Parser
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(err) => {
            eprintln!("Parser error: {}", err);
            process::exit(1);
        }
    };
    println!("✓ Parser: {} AST nodes generated", ast.len());

    // Compiler
    let mut compiler = Compiler::new();
    let bytecode = match compiler.compile(ast) {
        Ok(b) => b,
        Err(err) => {
            eprintln!("Compiler error: {}", err);
            process::exit(1);
        }
    };
    println!("✓ Compiler: {} bytes of bytecode generated", bytecode.code.len());
    println!("  - Constants: {}", bytecode.constants.len());

    // Write bytecode to file
    let file = match fs::File::create(output_file) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error creating output file '{}': {}", output_file, err);
            process::exit(1);
        }
    };

    let mut writer = BufWriter::new(file);
    match bytecode.serialize(&mut writer) {
        Ok(_) => {
            println!("{}", "-".repeat(60));
            println!("✓ Compilation successful!");
            println!("Output: {}", output_file);
        }
        Err(err) => {
            eprintln!("Error writing bytecode: {}", err);
            process::exit(1);
        }
    }
}

fn exec_bytecode(filename: &str) {
    println!("Executing bytecode: {}", filename);
    println!("{}", "-".repeat(60));

    // Read bytecode file
    let file = match fs::File::open(filename) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error opening file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    let mut reader = BufReader::new(file);
    let bytecode = match BytecodeProgram::deserialize(&mut reader) {
        Ok(b) => b,
        Err(err) => {
            eprintln!("Error reading bytecode: {}", err);
            process::exit(1);
        }
    };

    println!("✓ Bytecode loaded:");
    println!("  - Code size: {} bytes", bytecode.code.len());
    println!("  - Constants: {}", bytecode.constants.len());
    println!("{}", "-".repeat(60));

    // Execute with VM
    let mut vm = VM::new(bytecode);
    match vm.execute() {
        Ok(result) => {
            println!("{}", "-".repeat(60));
            println!("✓ Execution completed successfully");
            println!("Result: {:?}", result);
        }
        Err(err) => {
            eprintln!("{}", "-".repeat(60));
            eprintln!("VM error: {}", err);
            process::exit(1);
        }
    }
}
