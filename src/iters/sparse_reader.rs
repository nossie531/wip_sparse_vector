use crate::alias::*;
use crate::common::*;
use crate::prelude::*;
use crate::values::*;
use only_one::prelude::*;
use std::iter::FusedIterator;
use std::ops::Range;

#[derive(Debug)]
#[must_use = msg::iter_must_use!()]
pub struct SparseReader<'a, T>
where 
    T: PartialEq,
{
    idx_range: Range<usize>,
    map_range: One<MapRange<'a, T>>,
}

impl<'a, T> SparseReader<'a, T>
where 
    T: PartialEq,
{
    pub(crate) fn new(vec: &'a SparseVec<T>, range: Range<usize>) -> Self {
        Self {
            idx_range: range.clone(),
            map_range: One::new(vec.map.range(range))
        }
    }

    fn is_default(&self) -> bool {
        !One::exists(&self.map_range)
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
            idx_range: self.idx_range.clone(),
            map_range: self.map_range.clone(),
        }
    }
}

impl<T> Default for SparseReader<'_, T>
where 
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            idx_range: Default::default(),
            map_range: Default::default(),
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

        let kv = self.map_range.next()?;
        let offset = self.idx_range.start;
        Some(ElmReader::new(*kv.0 - offset, kv.1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_default() {
            return (0, Some(0));
        }

        self.map_range.size_hint()
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

        let kv = self.map_range.next_back()?;
        let offset = self.idx_range.start;
        Some(ElmReader::new(*kv.0 - offset, kv.1))
    }
}
