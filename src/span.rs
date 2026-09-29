use std::{
    fmt,
    ops::{AddAssign, Index, Range},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteIndex(u32);

impl ByteIndex {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for ByteIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<usize> for ByteIndex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<ByteIndex> for usize {
    fn from(value: ByteIndex) -> Self {
        value.0 as Self
    }
}

impl AddAssign for ByteIndex {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl fmt::Display for ByteIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct LineIndex {
    offsets: Vec<u32>,
    length: u32,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut offsets = Vec::with_capacity(source.bytes().filter(|&b| b == b'\n').count() + 1);

        offsets.push(0);

        for (offset, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                offsets.push((offset + 1) as u32);
            }
        }

        Self {
            offsets,
            length: source.len() as u32,
        }
    }

    pub fn line(&self, index: ByteIndex) -> usize {
        assert!(index.0 <= self.length);

        match self.offsets.binary_search(&index.0) {
            Ok(line) => line + 1,
            Err(line) => line,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    start: ByteIndex,
    end: ByteIndex,
}

impl Span {
    pub fn new(start: ByteIndex, end: ByteIndex) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn empty(at: ByteIndex) -> Self {
        Self::new(at, at)
    }

    pub const fn start(self) -> ByteIndex {
        self.start
    }

    pub const fn end(self) -> ByteIndex {
        self.end
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }

    pub const fn len(self) -> usize {
        (self.end.0 - self.start.0) as usize
    }

    pub const fn range(self) -> Range<usize> {
        self.start.0 as usize..self.end.0 as usize
    }
}

impl Index<Span> for str {
    type Output = Self;

    fn index(&self, span: Span) -> &Self::Output {
        &self[span.range()]
    }
}
