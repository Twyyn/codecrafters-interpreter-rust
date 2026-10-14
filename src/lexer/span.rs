use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharIndex {
    /// Zero-based byte offset into the complete UTF-8 source.
    pub offset: usize,
    /// One-based line number. CRLF counts as one line break.
    pub line: usize,
}

impl Default for CharIndex {
    fn default() -> Self {
        Self { offset: 0, line: 1 }
    }
}

impl CharIndex {
    pub fn new(offset: usize, line: usize) -> Self {
        Self { offset, line }
    }
}

impl fmt::Display for CharIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line: {} Byte offset: {}", self.line, self.offset)
    }
}
