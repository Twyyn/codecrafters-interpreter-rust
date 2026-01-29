use std::{env, fs, process};

#[allow(dead_code)]
pub struct Args {
    pub command: String,
    pub filename: String,
    pub source: String,
}

pub fn parse() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <command> <filename>", args[0]);
        process::exit(64);
    }

    let command = args[1].clone();
    let filename = args[2].clone();

    let source = fs::read_to_string(&filename).unwrap_or_else(|e| {
        eprintln!("Failed to read file {filename}: {e}");
        process::exit(66);
    });

    Args {
        command,
        filename,
        source,
    }
}
