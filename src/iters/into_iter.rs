//! Provider of [`IntoIter`].

use crate::prelude::*;
use only_one::prelude::*;
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::ops::Range;

/// An owning iterator over the elements of a [`SparseVec`].
///
/// This type is created by the [`into_iter`] method on
/// [`SparseVec`] (provided by the [`IntoIterator`] trait).
/// See its documentation for more.
///
/// [`into_iter`]: crate::SparseVec::into_iter
#[derive(Clone, Debug)]
pub struct IntoIter<T>
where
    T: PartialEq,
{
    /// Underlying collection.
    vec: One<SparseVec<T>>,

    /// Iterating range.
    range: Range<usize>,
}

impl<T> IntoIter<T>
where
    T: PartialEq,
{
    /// Creates a new instance.
    pub(crate) fn new(vec: SparseVec<T>) -> Self {
        let vec = One::new(vec);
        let range = 0..vec.len;
        Self { vec, range }
    }

    /// Returns `true` if the iterator is empty.
    fn is_end(&self) -> bool {
        self.size_hint().1.unwrap() == 0
    }
}

impl<T> Default for IntoIter<T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            vec: Default::default(),
            range: Default::default(),
        }
    }
}

impl<T> ExactSizeIterator for IntoIter<T>
where
    T: PartialEq,
{
    // nop.
}

impl<T> FusedIterator for IntoIter<T>
where
    T: PartialEq,
{
    // nop.
}

impl<T> Iterator for IntoIter<T>
where
    T: PartialEq,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }

        let removed = self.vec.take(self.range.start);
        self.range.start += 1;
        Some(removed)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.range.len(), Some(self.range.len()))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>
where
    T: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }

        let removed = self.vec.take(self.range.end - 1);
        self.range.end -= 1;
        Some(removed)
    }
}
