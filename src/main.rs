use codecrafters_interpreter::ast::AstPrinter;
use codecrafters_interpreter::lexer::Lexer;
use codecrafters_interpreter::parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut lexer = Lexer::new(&file_contents);
            let tokens = lexer.scan_tokens().to_vec();

            if lexer.had_error() {
                std::process::exit(65);
            }

            let mut parser = Parser::new(tokens);
            let expr = parser.parse().unwrap();
            let mut printer = AstPrinter;
            println!("{}", printer.print(&expr));
        }

        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
