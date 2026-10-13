use super::span::Span;
use std::{fmt, num::ParseFloatError};

#[derive(Debug, Clone)]
pub struct SpannedError<T> {
    pub error: T,
    pub span: Span,
}

impl<T> SpannedError<T> {
    pub fn new(error: T, span: Span) -> Self {
        Self { error, span }
    }
}

impl<T: fmt::Display> fmt::Display for SpannedError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.span.start().line, self.error)
    }
}

impl<T: std::error::Error + 'static> std::error::Error for SpannedError<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LexError {
    #[error("Unexpected character: {ch}")]
    UnexpectedChar { ch: char },

    #[error("Unterminated string.")]
    UnterminatedString,

    #[error("Invalid number '{text}': {error}")]
    InvalidNumber {
        #[source]
        error: ParseFloatError,
        text: String,
    },

    #[error("Number overflow: '{text}' is out of range")]
    NumberOverflow { text: String },
}
