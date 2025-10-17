use crate::msg;
use crate::values::ElmReader;
use only_one::prelude::*;
use pstd::collections::btree_map::Range;
use std::iter::FusedIterator;

#[derive(Debug)]
#[must_use = msg::iter_must_use!()]
pub struct SparseReader<'a, T> {
    offset: usize,
    range: One<Range<'a, usize, T>>,
}

impl<'a, T> SparseReader<'a, T> {
    pub(crate) fn new(offset: usize, range: Range<'a, usize, T>) -> Self {
        let range = One::new(range);
        Self { offset, range }
    }

    fn is_default(&self) -> bool {
        !One::exists(&self.range)
    }
}

/// Restricted implementation.
///
/// # TODO for future
///
/// Currently `T` requires [`Clone`]. This is current limitation.
impl<T> Clone for SparseReader<'_, T>
where
    T: PartialEq + Clone,
{
    fn clone(&self) -> Self {
        Self {
            offset: self.offset.clone(),
            range: self.range.clone(),
        }
    }
}

impl<T> Default for SparseReader<'_, T> {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            range: Default::default(),
        }
    }
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
        if self.is_default() {
            return None;
        }

        let kv = self.range.next();
        kv.map(|x| ElmReader::new(*x.0 - self.offset, x.1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_default() {
            return (0, Some(0));
        }

        self.range.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for SparseReader<'a, T>
where
    T: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_default() {
            return None;
        }

        let kv = self.range.next_back();
        kv.map(|x| ElmReader::new(*x.0 - self.offset, x.1))
    }
}
