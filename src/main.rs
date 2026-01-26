use codecrafters_interpreter::lexer::Lexer;
use std::env;
use std::fs;

//cargo run -- tokenize C:\Users\bayba\Desktop\test.lox
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut lexer = Lexer::new(&file_contents);
            let tokens = lexer.scan_tokens();

            for token in tokens {
                println!("{token}")
            }

            if lexer.had_error() {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
