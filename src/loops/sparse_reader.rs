use crate::msg;
use crate::values::ElmReader;
use only_one::prelude::*;
use pstd::collections::btree_map::Range;
use std::iter::FusedIterator;

#[derive(Debug)]
#[must_use = msg::iter_must_use!()]
pub struct SparseReader<'a, T> {
    range: One<Range<'a, usize, T>>,
}

impl<'a, T> SparseReader<'a, T> {
    pub(crate) fn new(range: Range<'a, usize, T>) -> Self {
        Self {
            range: One::new(range),
        }
    }

    fn iter(&self) -> &Range<'a, usize, T> {
        &self.range
    }

    fn iter_mut(&mut self) -> &mut Range<'a, usize, T> {
        &mut self.range
    }
}

/// Restricted implementation.
///
/// # TODO for future
///
/// Currently [`Iter<'a, K, V>`](Iter) of [`pstd`] implements [`Clone`]
/// only if `K` and `V` implements [`Clone`]. Therefore our `T` also
/// requireds [`Clone`].
impl<T> Clone for SparseReader<'_, T>
where
    T: PartialEq + Clone,
{
    fn clone(&self) -> Self {
        Self {
            range: self.range.clone(),
        }
    }
}

impl<T> Default for SparseReader<'_, T> {
    fn default() -> Self {
        Self {
            range: One::default(),
        }
    }
}

impl<T> ExactSizeIterator for SparseReader<'_, T>
where
    T: PartialEq,
{
    // nop.
}

impl<T> FusedIterator for SparseReader<'_, T>
where
    T: PartialEq,
{
    // nop.
}

impl<'a, T> Iterator for SparseReader<'a, T>
where
    T: PartialEq,
{
    type Item = ElmReader<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_mut().next().map(|x| ElmReader::new(*x.0, x.1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter().size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for SparseReader<'a, T>
where
    T: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter_mut()
            .next_back()
            .map(|x| ElmReader::new(*x.0, x.1))
    }
}
