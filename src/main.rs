use codecrafters_interpreter::{errors::InterpreterError, lexer::Lexer};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let mut args = env::args();
    let program = args.next();

    match (args.next(), args.next()) {
        (None, None) => {
            if let Err(e) = run_prompt() {
                eprintln!("{e}");
            }
        }

        (Some(command), Some(filename)) => {
            if let Err(e) = run_file(&command, &filename) {
                eprintln!("{e}");
            }
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
            let (tokens, had_error) = match Lexer::new(src).scan_tokens() {
                Ok(t) => (t, false),
                Err(t) => (t, true),
            };

            for token in tokens {
                println!("{token}");
            }

            Ok(had_error)
        }

        _ => Err(InterpreterError::UnknownCommand(command.into())),
    }
}

fn run_prompt() -> Result<(), InterpreterError> {
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

        let _had_error = run("tokenize", line)?;
    }

    Ok(())
}

fn run_file(command: &str, filename: &str) -> Result<(), InterpreterError> {
    let src =
        fs::read_to_string(filename).map_err(|e| InterpreterError::FileRead(filename.into(), e))?;

    let had_error = run(command, &src)?;

    if had_error {
        std::process::exit(65);
    }

    Ok(())
}
