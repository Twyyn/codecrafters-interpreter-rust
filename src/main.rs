use codecrafters_interpreter::{errors::InterpreterError, lexer::Lexer, parser::Parser};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() -> Result<(), InterpreterError> {
    let mut args = env::args();
    let program = args.next();

    match (args.next(), args.next()) {
        (Some(command), None) => run_prompt(&command),

        (Some(command), Some(filename)) => {
            if run_file(&command, &filename)? {
                std::process::exit(65);
            }

            Ok(())
        }

        _ => {
            eprintln!(
                "Usage: {} [tokenize <filename>]",
                program.unwrap_or_default()
            );
            std::process::exit(1);
        }
    }
}

#[allow(clippy::single_match_else)]
fn run(command: &str, src: &str) -> Result<bool, InterpreterError> {
    match command {
        "tokenize" => {
            let (tokens, _) = Lexer::new(src).scan_tokens();

            for token in tokens {
                println!("{token}");
            }

            Ok(had)
        }
        "parse" => {
            let (tokens, had_error) = Lexer::new(src).scan_tokens();
            let parse_error = match Parser::new(&tokens).parse() {
                Ok(expr) => {
                    println!("{expr}");
                    false
                }
                Err(e) => {
                    eprintln!("{e}");
                    true
                }
            };

            Ok(had_error || parse_error)
        }
        _ => Err(InterpreterError::UnknownCommand(command.into())),
    }
}

fn run_prompt(command: &str) -> Result<(), InterpreterError> {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        let bytes_read = stdin.read_line(&mut input)?;

        if bytes_read == 0 {
            break;
        }

        let line = input.trim_end();
        if line.is_empty() {
            continue;
        }

        run(command, line)?;
    }

    Ok(())
}

fn run_file(command: &str, filename: &str) -> Result<bool, InterpreterError> {
    let src =
        fs::read_to_string(filename).map_err(|e| InterpreterError::FileRead(filename.into(), e))?;

    run(command, &src)
}
