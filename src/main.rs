use std::env;
use std::fs;

use codecrafters_interpreter::Lexer;

fn main() {
    let mut args = env::args().skip(1);

    let Some(command) = args.next() else {
        eprintln!("Usage: tokenize <filename>");
        return;
    };

    let Some(filename) = args.next() else {
        eprintln!("Usage: tokenize <filename>");
        return;
    };

    match command.as_str() {
        "tokenize" => {
            let source = match fs::read_to_string(&filename) {
                Ok(source) => source,
                Err(err) => {
                    eprintln!("Failed to read {filename}: {err}");
                    return;
                }
            };

            for token in Lexer::new(&source) {
                match token {
                    Ok(token) => println!("{token:#?}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {command}");
        }
    }
}
