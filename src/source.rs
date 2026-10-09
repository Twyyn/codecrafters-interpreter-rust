use std::ops::{Add, AddAssign, Index};

// -------------------------------------------------------------------------------------------------
// ByteIndex
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteIndex(pub u32);

impl ByteIndex {
    pub fn new(index: usize) -> Self {
        debug_assert!(u32::try_from(index).is_ok());
        Self(index as u32)
    }
}

impl TryFrom<usize> for ByteIndex {
    type Error = std::num::TryFromIntError;

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(index)?))
    }
}

impl From<ByteIndex> for usize {
    fn from(index: ByteIndex) -> Self {
        index.0 as Self
    }
}

impl Index<ByteIndex> for [u8] {
    type Output = u8;

    fn index(&self, index: ByteIndex) -> &Self::Output {
        &self[usize::from(index)]
    }
}

impl Add<Self> for ByteIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<usize> for ByteIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 = (self.0 as usize + rhs) as u32;
    }
}

impl Index<ByteIndex> for str {
    type Output = Self;

    fn index(&self, index: ByteIndex) -> &Self::Output {
        &self[usize::from(index)..]
    }
}

// -------------------------------------------------------------------------------------------------
// Span
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: ByteIndex,
    pub end: ByteIndex,
}

impl Span {
    pub fn new(start: ByteIndex, end: ByteIndex) -> Self {
        Self { start, end }
    }
}

impl Index<Span> for str {
    type Output = Self;

    fn index(&self, span: Span) -> &Self::Output {
        &self[span.start.into()..span.end.into()]
    }
}

// -------------------------------------------------------------------------------------------------
// Source
// -------------------------------------------------------------------------------------------------

pub struct Source {
    text: String,
    line_offsets: Vec<u32>,
}

impl Source {
    pub fn new(text: String) -> Self {
        assert!(u32::try_from(text.len()).is_ok());

        let line_offsets = std::iter::once(0)
            .chain(
                text.bytes()
                    .enumerate()
                    .filter(|&(_, byte)| byte == b'\n')
                    .map(|(index, _)| (index + 1) as u32),
            )
            .collect();

        Self { text, line_offsets }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn line_number(&self, offset: ByteIndex) -> u32 {
        debug_assert!(
            usize::from(offset) <= self.text.len(),
            "offset out of bounds",
        );

        self.line_offsets
            .partition_point(|&start| start <= offset.0) as u32
    }
}
