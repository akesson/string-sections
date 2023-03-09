use std::ops::{Deref, Range};

use crate::LineSpan;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SectionSpan<'a> {
    pub start_line: LineSpan<'a>,
    pub end_line: LineSpan<'a>,
}

impl<'a> SectionSpan<'a> {
    /// Returns the byte index range of the start and
    /// end of the line, excluding the line ending
    /// part `\n` or `\r\n`.
    pub fn range(&self) -> Range<usize> {
        self.start_line.end..self.end_line.start
    }

    /// Returns `&str` of the line, excluding `\n` and `\r\n`.
    pub fn as_str(&self) -> &'a str {
        let lbr: &[_] = &['\n', '\r'];
        &self.start_line.text[self.range()].trim_matches(lbr)
    }
}

impl<'a> Deref for SectionSpan<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
