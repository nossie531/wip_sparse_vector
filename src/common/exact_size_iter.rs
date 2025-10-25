//! Provider of [`ExactSizeIter`].

use std::iter::FusedIterator;
use std::vec::IntoIter;

/// An iterator wrapper to implement [`ExactSizeIterator`].
///
/// If the target iterator's `size_hint` returns an accurate value,
/// use it. Otherwise, preload all items into an internal vector.
///
/// [`size_hint`]: Iterator::size_hint
pub struct ExactSizeIter<I>(Mode<I>)
where
    I: Iterator;

impl<I> ExactSizeIter<I>
where
    I: Iterator,
{
    /// Creates a new instance.
    pub fn new(iter: I) -> Self {
        let sh = iter.size_hint();
        if Some(sh.0) == sh.1 {
            Self(Mode::IterMode(iter))
        } else {
            Self(Mode::VecMode(Vec::from_iter(iter).into_iter()))
        }
    }
}

impl<I> DoubleEndedIterator for ExactSizeIter<I>
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

impl<I> ExactSizeIterator for ExactSizeIter<I>
where
    I: Iterator,
{
    // nop.
}

impl<I> FusedIterator for ExactSizeIter<I>
where
    I: Iterator,
{
    // nop.
}

impl<I> Iterator for ExactSizeIter<I>
where
    I: Iterator,
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

/// Iteration mode.
enum Mode<I: Iterator> {
    /// Base iterator mode.
    IterMode(I),
    /// Vector preloading mode.
    VecMode(IntoIter<I::Item>),
}
