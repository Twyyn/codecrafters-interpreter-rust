use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to read '{0}': {1}")]
    FileRead(String, #[source] std::io::Error),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("{0}")]
    Lex(#[from] crate::lexer::LexError),
}
