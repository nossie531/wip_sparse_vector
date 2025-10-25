//! Provider of [`SparseReader`].

use crate::aliases::*;
use crate::common::*;
use crate::prelude::*;
use crate::values::*;
use only_one::prelude::*;
use std::iter::FusedIterator;
use std::ops::Range;

/// A sparse iterator over the elements of a [`SparseVec`].
/// 
/// This type is created by [`SparseVec::sparse_reader`].
/// See its documentation for more.
#[derive(Debug)]
#[must_use = msg::iter_must_use!()]
pub struct SparseReader<'a, T>
where 
    T: PartialEq,
{
    /// Underlying sparse vector length.
    len: usize,

    /// Underlying sparse vector NNP.
    nnp: usize,

    /// Slicing range.
    idx_range: Range<usize>,

    /// Iterating range of underlying sparse vector map.
    map_range: One<MapRange<'a, T>>,
}

impl<'a, T> SparseReader<'a, T>
where 
    T: PartialEq,
{
    /// Creates a new instance.
    pub(crate) fn new(vec: &'a SparseVec<T>, range: Range<usize>) -> Self {
        Self {
            len: vec.len(),
            nnp: vec.nnp(),
            idx_range: range.clone(),
            map_range: One::new(vec.map.range(range))
        }
    }

    /// Returns `true` if this is default instance.
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
            len: self.len,
            nnp: self.nnp,
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
            len: Default::default(),
            nnp: Default::default(),
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

        let min = self.nnp.saturating_sub(self.len - self.idx_range.len());
        let max = usize::min(self.nnp, self.idx_range.len());
        (min, Some(max))
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
