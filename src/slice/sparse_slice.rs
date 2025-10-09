use std::ops::RangeBounds;
use crate::iter::{Iter, SparseReader};
use crate::slice::SparseVecPart;

pub trait SparseSlice<T>
where 
    T: PartialEq,
{
    /// Returns splice length.
    fn len(&self) -> usize;

    /// Slice this slice.
    fn slice<R>(&self, range: R) -> SparseVecPart<'_, T>
    where 
        R: RangeBounds<usize>;

    /// Returns an iterator.
    fn iter(&self) -> Iter<'_, T>;

    /// Returns none padding elements reader.
    fn sparse_reader(&self) -> SparseReader<'_, T>;

    /// Returns `true` if slice is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copies `self` into a new [`Vec`].
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        Vec::from_iter(self.iter().cloned())
    }
}
