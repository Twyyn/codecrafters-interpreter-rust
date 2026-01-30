mod cli;
use codecrafters_interpreter::{interpreter::Interpreter, lexer::Lexer, parser::Parser};
use std::process;

fn main() {
    let args = cli::parse();

    match args.command.as_str() {
        "tokenize" => {
            let result = Lexer::new(&args.source).scan_tokens();

            for token in &result.tokens {
                println!("{token}");
            }

            if !result.errors.is_empty() {
                for err in &result.errors {
                    eprintln!("{err}");
                }
                process::exit(65);
            }
        }

        "parse" => {
            let lex_result = Lexer::new(&args.source).scan_tokens();

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

        "evaluate" => {
            let lex_result = Lexer::new(&args.source).scan_tokens();

            if !lex_result.errors.is_empty() {
                for err in &lex_result.errors {
                    eprintln!("{err}");
                }
                process::exit(65);
            }

            let mut parser = Parser::new(lex_result.tokens);
            let mut interpreter = Interpreter::new();
            match parser.parse() {
                Ok(expr) => match interpreter.evaluate(expr) {
                    Ok(value) => println!("{}", value.as_string()),
                    Err(e) => {
                        eprintln!("{e}");
                        process::exit(70);
                    }
                },
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(65);
                }
            }
        }

        "run" => {
            let lex_result = Lexer::new(&args.source).scan_tokens();

            if !lex_result.errors.is_empty() {
                for err in &lex_result.errors {
                    eprintln!("{err}");
                }
                process::exit(65);
            }

            let mut parser = Parser::new(lex_result.tokens);
            let mut interpreter = Interpreter::new();
            let statements = match parser.parse_statements() {
                Ok(statements) => statements,
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(65);
                }
            };

            for statement in statements {
                if let Err(e) = interpreter.run(statement) {
                    eprintln!("{e}");
                    process::exit(70);
                }
            }
        }

        _ => {
            eprintln!("Unknown command: {}", args.command);
        }
    }
}
