use core::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteIndex(pub usize);

impl From<ByteIndex> for usize {
    fn from(value: ByteIndex) -> Self {
        value.0
    }
}

impl From<usize> for ByteIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl fmt::Display for ByteIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineIndex(pub usize);

impl LineIndex {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl From<LineIndex> for usize {
    fn from(value: LineIndex) -> Self {
        value.0
    }
}

impl From<usize> for LineIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl fmt::Display for LineIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    offset: ByteIndex,
    line: LineIndex,
}

impl Location {
    pub fn advance(&mut self, offset: ByteIndex, ch: char) {
        self.offset = offset;

        if ch == '\n' {
            self.line.increment();
        }
    }

    pub fn offset(&self) -> ByteIndex {
        self.offset
    }

    pub fn line(&self) -> LineIndex {
        self.line
    }
}
