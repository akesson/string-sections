mod line_span;
mod section_finder;
mod section_iter;
mod section_span;

pub mod prelude;

use std::ops::Range;

pub use line_span::LineSpan;
pub use section_finder::{SectionFinder, SectionFnFinder};
pub use section_iter::SectionIter;
pub use section_span::SectionSpan;

fn str_to_range_unchecked(string: &str, substring: &str) -> Range<usize> {
    let start = (substring.as_ptr() as usize) - (string.as_ptr() as usize);
    let end = start + substring.len();

    start..end
}
