use std::{iter::FusedIterator, str::Lines};

use crate::{section_span::SectionSpan, SectionFinder};

pub use super::LineSpan;

#[derive(Clone)]
pub struct SectionIter<'a, F: SectionFinder> {
    pub(crate) text: &'a str,
    pub(crate) iter: Lines<'a>,
    pub(crate) finder: F,
}

impl<'a, F: SectionFinder> SectionIter<'a, F> {
    fn next_start(&mut self) -> Option<LineSpan<'a>> {
        while let Some(line) = self.iter.next() {
            let line = LineSpan::new(self.text, line);

            if self.finder.is_start(&line) {
                return Some(line);
            }
        }
        None
    }

    fn forward_to_end(&mut self, section: &mut SectionSpan<'a>) -> bool {
        while let Some(line) = self.iter.next() {
            section.end_line = LineSpan::new(self.text, line);
            if self.finder.is_end(&section) {
                return true;
            }
        }
        false
    }
}

impl<'a, F: SectionFinder> Iterator for SectionIter<'a, F> {
    type Item = SectionSpan<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(start_line) = self.next_start() {
            let mut section = SectionSpan {
                start_line,
                end_line: start_line,
            };
            self.forward_to_end(&mut section);
            Some(section)
        } else {
            None
        }
    }
}

impl<'a, F: SectionFinder> FusedIterator for SectionIter<'a, F> {}
