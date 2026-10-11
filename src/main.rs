use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use codecrafters_interpreter::Lexer;
use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Tokenize { filepath: PathBuf },
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();

    match args.command {
        Commands::Tokenize { filepath } => tokenize(&filepath),
    }
}

fn tokenize(filepath: &Path) -> Result<ExitCode> {
    let source = fs::read_to_string(filepath)
        .with_context(|| format!("Unable to read source file: {}", filepath.display()))?;

    let mut had_errors = false;

    for result in Lexer::new(&source) {
        match result {
            Ok(token) => println!("{token}"),
            Err(error) => {
                eprintln!("{error}");
                had_errors = true;
            }
        }
    }

    if had_errors {
        return Ok(ExitCode::from(65));
    }

    Ok(ExitCode::SUCCESS)
}
