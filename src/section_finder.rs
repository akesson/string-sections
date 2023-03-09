use crate::{LineSpan, SectionSpan};

pub trait SectionFinder {
    fn is_start(&self, line: &LineSpan) -> bool;
    fn is_end(&self, section: &SectionSpan) -> bool;
}

pub struct SectionFnFinder<S, E>
where
    S: Fn(&LineSpan) -> bool,
    E: Fn(&SectionSpan) -> bool,
{
    start: S,
    end: E,
}

impl<S, E> SectionFinder for SectionFnFinder<S, E>
where
    S: Fn(&LineSpan) -> bool,
    E: Fn(&SectionSpan) -> bool,
{
    fn is_start(&self, line: &LineSpan) -> bool {
        (self.start)(line)
    }

    fn is_end(&self, section: &SectionSpan) -> bool {
        (self.end)(section)
    }
}
impl<S, E> SectionFnFinder<S, E>
where
    S: Fn(&LineSpan) -> bool,
    E: Fn(&SectionSpan) -> bool,
{
    pub fn new(start: S, end: E) -> Self {
        Self { start, end }
    }
}
