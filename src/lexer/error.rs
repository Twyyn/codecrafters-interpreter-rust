use std::{fmt, num::ParseFloatError};

// -------------------------------------------------------------------------------------------------
// Spanned Error (Error And It's Index (Or Line))
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SpannedError<T> {
    pub error: T,
    pub index: u32,
}

impl<T: fmt::Display> fmt::Display for SpannedError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.index, self.error)
    }
}

// -------------------------------------------------------------------------------------------------
// Lexer Errors
// -------------------------------------------------------------------------------------------------

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
