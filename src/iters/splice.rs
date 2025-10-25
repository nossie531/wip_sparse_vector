//! Provider of [`Splice`].

use crate::SparseVec;
use crate::common::ExactSizeIter;
use only_one::One;
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::ops::{Range, RangeBounds};

/// A splicing iterator for [`SparseVec`].
///
/// This type is created by [`SparseVec::splice`].
/// See its documentation for more.
#[derive(Debug)]
pub struct Splice<'a, I>
where
    I: Iterator + 'a,
    I::Item: PartialEq,
{
    /// Underlying sparse vector.
    vec: &'a mut SparseVec<I::Item>,

    /// Range for remove.
    range: Range<usize>,

    /// Iterating edges for remove range.
    edges: Range<usize>,

    /// New values provider.
    iter: One<I>,

    /// Original length of underlying sparse vector.
    original_len: usize,
}

impl<'a, I> Splice<'a, I>
where
    I: Iterator + 'a,
    I::Item: PartialEq,
{
    pub(crate) fn new(vec: &'a mut SparseVec<I::Item>, range: Range<usize>, iter: I) -> Self {
        let edges = range.clone();
        let original_len = vec.len();
        let iter = One::new(iter);
        Self {
            vec,
            range,
            edges,
            iter,
            original_len,
        }
    }
}

impl<'a, I> Drop for Splice<'a, I>
where
    I: Iterator + 'a,
    I::Item: PartialEq,
{
    fn drop(&mut self) {
        let iter = ExactSizeIter::new(One::take(&mut self.iter));
        let diff = iter.len() as isize - self.range.len() as isize;
        let start = self.range.start_bound();
        let cursor = self.vec.map.lower_bound_mut(start);
        self.vec.len = (self.original_len as isize + diff) as usize;

        #[allow(unused_unsafe)]
        unsafe {
            let mut cursor = cursor.with_mutable_key();
            while let Some(elm) = cursor.next() {
                if *elm.0 < self.range.end {
                    cursor.remove_prev();
                } else {
                    let entry = cursor.peek_prev().unwrap();
                    let pos_ref = entry.0;
                    *pos_ref = (*pos_ref as isize + diff) as usize;
                }
            }
        }

        for (i, item) in iter.enumerate() {
            let pos = self.range.start + i;
            self.vec.map.insert(pos, item);
        }
    }
}

impl<'a, I> DoubleEndedIterator for Splice<'a, I>
where
    I: Iterator + 'a,
    I::Item: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.edges.is_empty() {
            return None;
        }

        let ret = self.vec.map.remove(&(self.edges.end - 1));
        self.vec.len -= 1;
        self.edges.end -= 1;
        Some(ret.unwrap_or(self.vec.clone_padding()))
    }
}

impl<'a, I> ExactSizeIterator for Splice<'a, I>
where
    I: Iterator + 'a,
    I::Item: PartialEq,
{
    // nop.
}

impl<'a, I> FusedIterator for Splice<'a, I>
where
    I: Iterator + 'a,
    I::Item: PartialEq,
{
    // nop.
}

impl<'a, I> Iterator for Splice<'a, I>
where
    I: Iterator + 'a,
    I::Item: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.edges.is_empty() {
            return None;
        }

        let ret = self.vec.map.remove(&self.edges.start);
        self.vec.len -= 1;
        self.edges.start += 1;
        Some(ret.unwrap_or(self.vec.clone_padding()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.edges.len(), Some(self.edges.len()))
    }
}
