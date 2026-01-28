use codecrafters_interpreter::lexer::Lexer;
use codecrafters_interpreter::parser::Parser;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <filename>", args[0]);
        process::exit(64);
    }

    let command = &args[1];
    let filename = &args[2];

    let source = fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Failed to read file {filename}: {e}");
        process::exit(66);
    });

    match command.as_str() {
        "tokenize" => {
            let result = Lexer::new(&source).scan_tokens();

            for token in &result.tokens {
                println!("{token}"); // assuming Display impl on Token
            }

            if !result.errors.is_empty() {
                for err in &result.errors {
                    eprintln!("{err}");
                }
                process::exit(65);
            }
        }

        "parse" => {
            let lex_result = Lexer::new(&source).scan_tokens();

            if !lex_result.errors.is_empty() {
                for err in &lex_result.errors {
                    eprintln!("{err}");
                }
                process::exit(65);
            }

            let mut parser = Parser::new(lex_result.tokens);
            match parser.parse() {
                Ok(expr) => println!("{expr}"),
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(65);
                }
            }
        }

        _ => {
            eprintln!("Unknown command: {command}");
            process::exit(64);
        }
    }
}
