use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to read '{0}': {1}")]
    FileRead(String, #[source] std::io::Error),

    #[error("[line {line}] Error: {message}")]
    Lex { line: usize, message: String },

    #[error("Unknown command: {0}")]
    UnknownCommand(String),
}
