use sparse_vector::prelude::*;
use std::ops::Range;

pub struct SliceContext {
    vec: SparseVec<i32>,
    range: Range<usize>,
}

impl SliceContext {
    pub fn new(vec: SparseVec<i32>, range: Range<usize>) -> Self {
        Self { vec, range }
    }

    pub fn fetch(&self) -> SparseSlice<'_, i32> {
        self.vec.slice(self.range.clone())
    }

    pub fn fetch_mut(&mut self) -> SparseSliceMut<'_, i32> {
        self.vec.slice_mut(self.range.clone())
    }
}
