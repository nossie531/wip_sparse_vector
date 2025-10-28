//! Provider of [`SparseWriterMap`].

use crate::iters::*;
use std::iter::FusedIterator;

/// Sparse writer item mapper.
///
/// This type is created by [`SparseWriter::map`].
/// See its documentation for more.
pub struct SparseWriterMap<'a, T, F>
where
    T: PartialEq,
{
    // Base object.
    base: SparseWriter<'a, T>,

    // Mapping closure.
    f: F,
}

impl<'a, T, F> SparseWriterMap<'a, T, F>
where
    T: PartialEq,
{
    pub(crate) fn new(base: SparseWriter<'a, T>, f: F) -> Self {
        Self { base, f }
    }
}

impl<B, T, F> DoubleEndedIterator for SparseWriterMap<'_, T, F>
where
    T: PartialEq,
    F: FnMut((usize, &mut T)) -> B,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        Some((self.f)(self.base.next_back()?))
    }
}

impl<B, T, F> ExactSizeIterator for SparseWriterMap<'_, T, F>
where
    T: PartialEq,
    F: FnMut((usize, &mut T)) -> B,
{
    // nop.
}

impl<B, T, F> FusedIterator for SparseWriterMap<'_, T, F>
where
    T: PartialEq,
    F: FnMut((usize, &mut T)) -> B,
{
    // nop.
}

impl<B, T, F> Iterator for SparseWriterMap<'_, T, F>
where
    T: PartialEq,
    F: FnMut((usize, &mut T)) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.f)(self.base.next()?))
    }
}
