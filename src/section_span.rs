use std::ops::{Deref, Range};

use crate::LineSpan;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SectionSpan<'a> {
    pub start_line: LineSpan<'a>,
    pub end_line: LineSpan<'a>,
}

const LBR: &[char] = &['\n', '\r'];

impl<'a> SectionSpan<'a> {
    /// Returns the byte index range of the section, excluding the start and end lines
    pub fn inner_range(&self) -> Range<usize> {
        self.start_line.end..self.end_line.start
    }

    /// Returns the byte index range of the section, including the start and end lines
    pub fn outer_range(&self) -> Range<usize> {
        self.start_line.start..self.end_line.end
    }

    /// Returns section content as `&str`
    pub fn as_str(&self) -> &'a str {
        &self.start_line.text[self.inner_range()].trim_matches(LBR)
    }
}

impl<'a> Deref for SectionSpan<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
