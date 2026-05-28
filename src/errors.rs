use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Failed to read '{0}': {1}")]
    FileRead(String, #[source] std::io::Error),
    #[error("Parse error on line {line}: {message}")]
    Parse { line: usize, message: String },
}
