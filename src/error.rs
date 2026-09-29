use crate::{LineIndex, Span};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    LexError(#[from] LexError),
}

#[derive(Debug)]
pub struct DiagnosticReport {
    pub error: Error,
    pub span: Span,
}

impl DiagnosticReport {
    pub fn report(&self, index: &LineIndex) -> String {
        format!("[line {}] {}", index.line(self.span.start()), self.error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LexError {
    #[error("Unexpected character: {ch}")]
    UnexpectedChar { ch: char },

    #[error("Unterminated string.")]
    UnterminatedString,
}
