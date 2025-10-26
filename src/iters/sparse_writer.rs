//! Provider of [`SparseWriter`].

use crate::aliases::*;
use crate::common::*;
use crate::prelude::*;
use only_one::prelude::*;
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::ops::{Bound, Range};

/// A mutable sparse iterator over the elements of a [`SparseVec`].
///
/// This type is created by [`SparseVec::sparse_writer`].
/// See its documentation for more.
#[derive(Debug)]
#[must_use = msg::iter_must_use!()]
pub struct SparseWriter<'a, T>
where
    T: PartialEq,
{
    /// Underlying sparse vector length.
    len: usize,

    /// Underlying sparse vector NNP.
    nnp: usize,

    /// Padding value reference.
    padding: One<&'a T>,

    /// Slicing range.
    idx_range: Range<usize>,

    /// Iterating range of underlying sparse vector map.
    map_range: One<MapRangeMut<'a, T>>,

    /// Map pointer (Used only after [`Self::map_range`] is droped.)
    map: *mut Map<T>,
}

impl<'a, T> SparseWriter<'a, T>
where
    T: PartialEq,
{
    /// Creates a new instance.
    pub(crate) fn new(vec: &'a mut SparseVec<T>, range: Range<usize>) -> Self {
        let map_ptr = (&mut vec.map) as *mut _;
        Self {
            len: vec.len(),
            nnp: vec.nnp(),
            padding: One::new(vec.padding.refs()),
            idx_range: range.clone(),
            map_range: One::new(vec.map.range_mut(range)),
            map: map_ptr,
        }
    }

    /// Returns `true` if this is default instance.
    fn is_default(&self) -> bool {
        !One::exists(&self.map_range)
    }
}

impl<T> Default for SparseWriter<'_, T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            len: Default::default(),
            nnp: Default::default(),
            padding: Default::default(),
            idx_range: Default::default(),
            map_range: Default::default(),
            map: Default::default(),
        }
    }
}

impl<T> Drop for SparseWriter<'_, T>
where
    T: PartialEq,
{
    fn drop(&mut self) {
        if One::exists(&self.map_range) {
            One::take(&mut self.map_range);
        }

        if !self.map.is_null() {
            let map = unsafe { &mut *self.map };
            let start = Bound::Included(&self.idx_range.start);
            let cursor = &mut map.lower_bound_mut(start);
            while let Some(elm) = cursor.next() {
                if *elm.0 >= self.idx_range.end {
                    break;
                }

                if *elm.1 == **self.padding {
                    cursor.remove_prev();
                }
            }
        }
    }
}

impl<T> FusedIterator for SparseWriter<'_, T>
where
    T: PartialEq,
{
    // nop.
}

impl<'a, T> Iterator for SparseWriter<'a, T>
where
    T: PartialEq,
{
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_default() {
            return None;
        }

        let kv = self.map_range.next()?;
        let offset = self.idx_range.start;
        Some((*kv.0 - offset, kv.1))
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

impl<'a, T> DoubleEndedIterator for SparseWriter<'a, T>
where
    T: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_default() {
            return None;
        }

        let kv = self.map_range.next_back()?;
        let offset = self.idx_range.start;
        Some((*kv.0 - offset, kv.1))
    }
}
