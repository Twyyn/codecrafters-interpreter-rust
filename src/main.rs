#![allow(unused_variables)]
use codecrafters_interpreter::{errors::InterpreterError, lexer::Lexer};
use std::env;
use std::fs;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}

fn run() -> Result<(), InterpreterError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename)
                .map_err(|e| InterpreterError::FileRead(filename.into(), e))?;

            for token in Lexer::new(&file_contents).scan_tokens() {
                println!("{token}");
            }
        }

        _ => {
            eprintln!("Unknown command: {command}");
        }
    }

    Ok(())
}
