use anyhow::{Context, Result, bail};
use codecrafters_interpreter::Lexer;
use std::{env, fs};

fn main() -> Result<()> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "program".into());

    let (Some(command), Some(filename)) = (args.next(), args.next()) else {
        bail!("Usage: {program} tokenize <filename>")
    };

    if command != "tokenize" {
        bail!("Unknown command: {command}")
    }

    let source =
        fs::read_to_string(&filename).with_context(|| format!("Failed to read {filename}"))?;

    let mut had_error = false;
    for result in Lexer::new(&source) {
        match result {
            Ok(token) => println!("{token}"),
            Err(error_token) => {
                had_error = true;
                eprintln!("{error_token}");
            }
        }
    }

    println!("EOF  null");

    if had_error {
        std::process::exit(65);
    }

    Ok(())
}
