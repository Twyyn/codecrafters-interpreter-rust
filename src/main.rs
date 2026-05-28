use codecrafters_interpreter::{errors::InterpreterError, lexer::Lexer};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let command = &args[1];
            if let Err(e) = run_prompt(command) {
                eprintln!("{e}");
            }
        }
        3 => {
            let command = &args[1];
            let filename = &args[2];

            if let Err(e) = run_file(command, filename) {
                eprintln!("{e}");
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Usage: {} [tokenize <filename>]", args[0]);
            std::process::exit(1);
        }
    }
}

#[allow(clippy::single_match_else)]
fn run(command: &str, src: &str) -> Result<(), InterpreterError> {
    match command {
        "tokenize" => {
            let tokens = Lexer::new(src).scan_tokens()?;
            for token in tokens {
                println!("{token}");
            }

            Ok(())
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

        if let Err(e) = run(command, line) {
            eprintln!("{e}");
        }
    }

    Ok(())
}

fn run_file(command: &str, filename: &str) -> Result<(), InterpreterError> {
    let src =
        fs::read_to_string(filename).map_err(|e| InterpreterError::FileRead(filename.into(), e))?;

    run(command, &src)
}
