use sparse_vector::prelude::*;
use std::ops::Range;

pub struct SliceContext<T>
where
    T: PartialEq,
{
    vec: SparseVec<T>,
    range: Range<usize>,
}

impl<T> SliceContext<T>
where
    T: PartialEq,
{
    pub fn new(vec: SparseVec<T>, range: Range<usize>) -> Self {
        Self { vec, range }
    }

    pub fn vec(&self) -> &SparseVec<T> {
        &self.vec
    }

    pub fn fetch(&self) -> SparseSlice<'_, T> {
        self.vec.slice(self.range.clone())
    }

    pub fn fetch_mut(&mut self) -> SparseSliceMut<'_, T> {
        self.vec.slice_mut(self.range.clone())
    }
}
