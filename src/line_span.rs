use std::{
    fmt,
    ops::{Deref, Range},
};

use crate::str_to_range_unchecked;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LineSpan<'a> {
    pub(crate) text: &'a str,
    pub start: usize,
    pub end: usize,
}

impl<'a> LineSpan<'a> {
    pub fn new(text: &'a str, substring: &str) -> Self {
        let Range { start, end } = str_to_range_unchecked(text, substring);
        LineSpan { text, start, end }
    }

    /// Returns the byte index range of the start and
    /// end of the line, excluding the line ending
    /// part `\n` or `\r\n`.
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns `&str` of the line, excluding `\n` and `\r\n`.
    pub fn as_str(&self) -> &'a str {
        &self.text[self.range()]
    }
}

impl<'a> Deref for LineSpan<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> From<LineSpan<'a>> for &'a str {
    fn from(span: LineSpan<'a>) -> &'a str {
        span.as_str()
    }
}

impl<'a> From<LineSpan<'a>> for Range<usize> {
    fn from(span: LineSpan<'a>) -> Range<usize> {
        span.range()
    }
}

impl<'a> fmt::Debug for LineSpan<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("LineSpan")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("line", &self.as_str())
            .finish()
    }
}

impl<'a> fmt::Display for LineSpan<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(fmt)
    }
}
