use std::{iter::FusedIterator, vec::IntoIter};

pub struct SizedIter<I>(Mode<I>)
where 
    I: Iterator;

impl<I> SizedIter<I>
where 
    I: Iterator
{
    pub fn new(iter: I) -> Self {
        let sh = iter.size_hint();
        if Some(sh.0) == sh.1 {
            Self(Mode::IterMode(iter))
        } else {
            Self(Mode::VecMode(Vec::from_iter(iter).into_iter()))
        }
    }
}

impl<I> DoubleEndedIterator for SizedIter<I>
where 
    I: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Mode::IterMode(x) => x.next_back(),
            Mode::VecMode(x) => x.next_back(),
        }
    }
}

impl<I> ExactSizeIterator for SizedIter<I>
where 
    I: Iterator,
{
    // nop.
}

impl<I> FusedIterator for SizedIter<I>
where 
    I: Iterator,
{
    // nop.
}

impl<I> Iterator for SizedIter<I>
where 
    I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Mode::IterMode(x) => x.next(),
            Mode::VecMode(x) => x.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            Mode::IterMode(x) => x.size_hint(),
            Mode::VecMode(x) => x.size_hint(),
        }
    }
}

enum Mode<I: Iterator> {
    IterMode(I),
    VecMode(IntoIter<I::Item>)
}
