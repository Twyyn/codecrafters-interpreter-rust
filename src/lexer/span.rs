use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index {
    /// Zero-based byte offset into the complete UTF-8 source, not a column.
    pub offset: usize,
    /// One-based line number. CRLF counts as one line break.
    pub line: usize,
}

impl Default for Index {
    fn default() -> Self {
        Self { offset: 0, line: 1 }
    }
}

impl Index {
    pub fn new(offset: usize, line: usize) -> Self {
        Self { offset, line }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line: {} Byte offset: {}", self.line, self.offset)
    }
}

/// A half-open source range: the start is included and the end is excluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: Index,
    end: Index,
}

impl Span {
    pub fn new(start: Index, end: Index) -> Self {
        Self { start, end }
    }

    pub const fn start(&self) -> Index {
        self.start
    }

    pub const fn end(&self) -> Index {
        self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
