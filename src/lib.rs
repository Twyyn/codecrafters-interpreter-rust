pub mod lexer;

use memchr::memchr_iter;
use std::{
    convert::From,
    fmt,
    ops::{Add, Sub},
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: ByteOffset,
    pub end: ByteOffset,
}

impl Span {
    pub fn new(start: ByteOffset, end: ByteOffset) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
pub struct SourceText {
    text: Arc<str>,
    line_starts: Vec<ByteOffset>,
}

impl SourceText {
    pub fn new(text: impl Into<Arc<str>>) -> Self {
        let text = text.into();

        let line_starts = std::iter::once(0)
            .chain(memchr_iter(b'\n', text.as_bytes()).map(|i| i + 1))
            .map(ByteOffset::from)
            .collect();

        Self { text, line_starts }
    }

    pub fn lookup_line_index(&self, offset: ByteOffset) -> Option<usize> {
        self.line_starts
            .partition_point(|&line_start| line_start <= offset)
            .checked_sub(1)
    }

    pub fn lookup_source_position(&self, offset: ByteOffset) -> Option<(usize, CharIndex)> {
        let text = self.text.as_ref();
        let byte_offset = usize::from(offset);

        if !text.is_char_boundary(byte_offset) {
            return None;
        }

        let line = self.lookup_line_index(offset)?;
        let line_start = usize::from(self.line_starts[line]);
        let column = text[line_start..byte_offset].chars().count();

        Some((line, CharIndex::from(column)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharIndex(pub usize);

impl fmt::Display for CharIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for CharIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteOffset(pub u32);

impl From<ByteOffset> for usize {
    fn from(index: ByteOffset) -> Self {
        index.0 as Self
    }
}

impl From<usize> for ByteOffset {
    fn from(value: usize) -> Self {
        assert!(u32::try_from(value).is_ok());
        Self(value as u32)
    }
}

impl Add for ByteOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for ByteOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl fmt::Display for ByteOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
