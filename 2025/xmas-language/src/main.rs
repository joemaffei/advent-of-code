/// Main entry point for the xmas language compiler/interpreter.
///
/// Usage:
///   xmas <file.xmas>              Run a xmas program file
///   xmas <file.xmas> -i input.txt  Run with input file
///   xmas                            Run from stdin

use std::env;
use std::fs;
use std::io::{self, Read};
use xmas_language::{Lexer, Parser, Interpreter};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Read source code
    let code = if args.len() > 1 && !args[1].starts_with('-') {
        // Read from file
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", filename, e);
                std::process::exit(1);
            }
        }
    } else {
        // Read from stdin
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Check for input file option
    let input_file = args.iter().position(|a| a == "-i" || a == "--input")
        .and_then(|i| args.get(i + 1));

    // Check for debug flag
    let debug = args.iter().any(|a| a == "-d" || a == "--debug");

    // Step 1: Tokenize
    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();

    // Step 2: Parse
    let mut parser = Parser::new(tokens, code.clone());
    let program = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Step 3: Interpret
    let mut interpreter = Interpreter::new();
    interpreter.set_debug(debug);

    // Set input if provided
    if let Some(input_path) = input_file {
        match fs::read_to_string(input_path) {
            Ok(input_content) => {
                interpreter.set_input(&input_content);
            }
            Err(e) => {
                eprintln!("Warning: Could not read input file '{}': {}", input_path, e);
            }
        }
    }

    match interpreter.interpret(&program) {
        Ok(result) => {
            // Print the result if it's meaningful
            if !matches!(result, xmas_language::Value::Array1D(ref arr) if arr.is_empty()) {
                println!("{}", format_value(&result));
            }
        }
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Format a value for display
fn format_value(value: &xmas_language::Value) -> String {
    match value {
        xmas_language::Value::Number(n) => {
            format!("{}", n)
        }
        xmas_language::Value::Boolean(b) => {
            format!("{}", b)
        }
        xmas_language::Value::String(s) => s.clone(),
        xmas_language::Value::Array1D(arr) => {
            let items: Vec<String> = arr.iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        }
        xmas_language::Value::Array2D(arr) => {
            let rows: Vec<String> = arr.iter()
                .map(|row| {
                    let items: Vec<String> = row.iter().map(format_value).collect();
                    format!("[{}]", items.join(", "))
                })
                .collect();
            format!("[{}]", rows.join(", "))
        }
    }
}
