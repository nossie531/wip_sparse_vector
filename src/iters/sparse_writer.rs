//! Provider of [`SparseWriter`].

use crate::aliases::*;
use crate::common::*;
use crate::iters::*;
use crate::prelude::*;
use only_one::prelude::*;
use std::fmt::Debug;
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
    /// Returns item mapper.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut vec = SparseVec::from_iter([1, 0, 3, 0, 5]);
    /// let mut writer = vec.sparse_writer();
    /// let map = writer.map(|x| (x.0, *x.1));
    /// assert!(map.eq([(0, 1), (2, 3), (4, 5)]));
    /// ```
    pub fn map<B, F>(self, f: F) -> SparseWriterMap<'a, T, F>
    where
        F: FnMut((usize, &mut T)) -> B,
    {
        SparseWriterMap::new(self, f)
    }

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

/// Methods like normal iterator.
impl<'a, T> SparseWriter<'a, T>
where
    T: PartialEq,
{
    /// Advances the iterator and returns the next value.
    ///
    /// This method is similar to [`Iterator::next`].
    /// See its documentation for more.
    pub fn next(&mut self) -> Option<(usize, &mut T)> {
        if self.is_default() {
            return None;
        }

        let kv = self.map_range.next()?;
        let offset = self.idx_range.start;
        Some((*kv.0 - offset, kv.1))
    }

    /// Returns the nth element from the end of the iterator.
    ///
    /// This method is similar to [`DoubleEndedIterator::next_back`].
    /// See its documentation for more.
    pub fn next_back(&mut self) -> Option<(usize, &mut T)> {
        if self.is_default() {
            return None;
        }

        let kv = self.map_range.next_back()?;
        let offset = self.idx_range.start;
        Some((*kv.0 - offset, kv.1))
    }

    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// This method is similar to [`Iterator::size_hint`].
    /// See its documentation for more.
    pub fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_default() {
            return (0, Some(0));
        }

        let min = self.nnp.saturating_sub(self.len - self.idx_range.len());
        let max = usize::min(self.nnp, self.idx_range.len());
        (min, Some(max))
    }

    /// Consumes the iterator, counting the number of iterations and returning it.
    ///
    /// This method is similar to [`Iterator::count`].
    /// See its documentation for more.
    pub fn count(self) -> usize {
        self.map(|_| ()).count()
    }

    /// Returns the `n`th element of the iterator.
    ///
    /// This method is similar to [`Iterator::nth`].
    /// See its documentation for more.
    pub fn nth(&mut self, n: usize) -> Option<(usize, &mut T)> {
        for _ in 0..n {
            self.next()?;
        }

        self.next()
    }

    /// Returns the `n`th element from the end of the iterator.
    ///
    /// This method is similar to [`DoubleEndedIterator::nth_back`].
    /// See its documentation for more.
    pub fn nth_back(&mut self, n: usize) -> Option<(usize, &mut T)> {
        for _ in 0..n {
            self.next_back()?;
        }

        self.next_back()
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
            // TODO: 計算時間！idx_range と map_range の間だけ消せばいい。
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
