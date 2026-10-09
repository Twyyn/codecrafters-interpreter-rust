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

    for token in Lexer::new(&source) {
        println!("{token:#?}");
    }

    println!("EOF  null");

    Ok(())
}
