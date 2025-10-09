use std::cmp::Ordering;
use std::mem;
use std::ops::{Index, Range, RangeBounds};
use crate::prelude::*;
use crate::slice::SparseVecPart;
use crate::iter::Iter;

#[repr(C)]
#[derive(Debug, Eq, Hash)]
pub struct SparseVecPartMut<'a, T>
where
    T: PartialEq,
{
    vec: &'a mut SparseVec<T>,
    range: Range<usize>,
}

impl<'a, T> SparseVecPartMut<'a, T>
where
    T: PartialEq,
{
    pub fn slice_ref(&self) -> &SparseVecPart<'_, T> {
        unsafe { mem::transmute(self) }
    }

    pub(crate) fn new(vec: &'a mut SparseVec<T>, range: Range<usize>) -> Self {
        assert!(range.end <= vec.len);
        Self { vec, range }
    }
}

impl<'a, T> SparseSlice<T> for SparseVecPartMut<'a, T>
where
    T: PartialEq,
{
    fn len(&self) -> usize {
        self.slice_ref().len()
    }

    fn slice<R>(&self, range: R) -> SparseVecPart<'_, T>
    where 
        R: RangeBounds<usize>
    {
        self.slice_ref().slice(range)
    }

    fn iter(&self) -> crate::Iter<'_, T> {
        self.slice_ref().iter()
    }

    fn sparse_reader(&self) -> crate::SparseReader<'_, T> {
        self.slice_ref().sparse_reader()
    }
}

impl<'a, T> Index<usize> for SparseVecPartMut<'a, T>
where 
    T: PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.range.len());
        self.slice_ref().index(index)
    }
}

impl<'a, T> IntoIterator for &'a SparseVecPartMut<'a, T>
where 
    T: PartialEq,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice_ref().into_iter()
    }
}

impl<'a, T> Ord for SparseVecPartMut<'a, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.slice_ref().cmp(other.slice_ref())
    }
}

impl<'a, T> PartialEq for SparseVecPartMut<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.slice_ref().eq(other.slice_ref())
    }
}

impl<'a, T> PartialOrd for SparseVecPartMut<'a, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.slice_ref().partial_cmp(other.slice_ref())
    }
}