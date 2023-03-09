use crate::{LineSpan, SectionFinder, SectionFnFinder, SectionIter, SectionSpan};

pub trait Sections {
    fn sections_find<'a, F: SectionFinder>(&'a self, finder: F) -> SectionIter<'a, F>;
    fn sections<'a, S, E>(&'a self, start: S, end: E) -> SectionIter<'a, SectionFnFinder<S, E>>
    where
        S: Fn(&LineSpan) -> bool,
        E: Fn(&SectionSpan) -> bool;
}

impl Sections for str {
    fn sections_find<'a, F: SectionFinder>(&'a self, finder: F) -> SectionIter<'a, F> {
        SectionIter {
            text: self,
            iter: self.lines(),
            finder,
        }
    }

    fn sections<'a, S, E>(&'a self, start: S, end: E) -> SectionIter<'a, SectionFnFinder<S, E>>
    where
        S: Fn(&LineSpan) -> bool,
        E: Fn(&SectionSpan) -> bool,
    {
        SectionIter {
            text: self,
            iter: self.lines(),
            finder: SectionFnFinder::new(start, end),
        }
    }
}
