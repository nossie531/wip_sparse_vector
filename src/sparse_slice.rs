use std::cmp::Ordering;
use std::ops::{Index, Range, RangeBounds};
use crate::prelude::*;
use crate::util;
use crate::iter::{Iter, SparseReader};
use crate::values::ElmReader;

#[repr(C)]
#[derive(Debug, Eq, Hash)]
pub struct SparseSlice<'a, T>
where
    T: PartialEq,
{
    vec: &'a SparseVec<T>,
    range: Range<usize>,
}

impl<'a, T> SparseSlice<'a, T>
where
    T: PartialEq,
{
    /// Returns `true` if slice is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns splice length.
    pub fn len(&self) -> usize {
        self.range.len()
    }    

    /// Returns an iterator.
    pub fn iter(&self) -> crate::Iter<'_, T> {
        Iter::new(self.vec, self.range.clone())
    }

    /// Returns none padding elements reader.
    pub fn sparse_reader(&self) -> crate::SparseReader<'_, T> {
        SparseReader::new(self.vec.map.range(self.range.clone()))
    }

    /// Copies `self` into a new [`Vec`].
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        Vec::from_iter(self.iter().cloned())
    }

    /// Slice this slice.
    pub fn slice<R>(&self, range: R) -> SparseSlice<'_, T>
    where 
        R: RangeBounds<usize>,
    {
        let range = util::to_index_range(range, self.range.len());

        Self {
            vec: self.vec,
            range
        }
    }

    pub(crate) fn new(vec: &'a SparseVec<T>, range: Range<usize>) -> Self {
        assert!(range.end <= vec.len);
        Self { vec, range }
    }
}

impl<'a, T> Index<usize> for SparseSlice<'a, T>
where 
    T: PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.range.len());
        let index = index + self.range.start;
        self.vec.map.get(&index).unwrap_or(&self.vec.padding)
    }
}

impl<'a, T> IntoIterator for &'a SparseSlice<'a, T>
where 
    T: PartialEq,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Ord for SparseSlice<'a, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a, T> PartialEq for SparseSlice<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.range.len() != other.range.len() {
            return false;
        }

        // Prepare common values.
        let len = self.range.len();
        let s_padding = &self.vec.padding;
        let o_padding = &other.vec.padding;
        let s_start = self.range.start;
        let o_start = self.range.start;

        // Prepare loop variables.
        let mut i = 0;
        let mut s_reader = self.sparse_reader();
        let mut o_reader = other.sparse_reader();
        let mut s_memo = None as Option<ElmReader<'_, T>>;
        let mut o_memo = None as Option<ElmReader<'_, T>>;

        // Loop shared part.
        while i < len {
            // Update memos for index.
            let s_index = s_start + i;
            let o_index = o_start + i;
            let s_fresh = s_memo.as_ref().is_some_and(|x| x.index() > s_index);
            let o_fresh = o_memo.as_ref().is_some_and(|x| x.index() > o_index);
            s_memo = if s_fresh { s_memo } else { s_reader.next() };
            o_memo = if o_fresh { o_memo } else { o_reader.next() };

            // Update indexs.
            let s_i = s_memo.as_ref().map(|x| x.index() - s_start).unwrap_or(len);
            let o_i = o_memo.as_ref().map(|x| x.index() - o_start).unwrap_or(len);
            let c_i = usize::min(s_i, o_i);
            let s_hit = c_i == s_i;
            let o_hit = c_i == o_i;

            // Update values.
            let s_value = s_memo.as_ref().map(|x| x.value()).unwrap_or(&self.vec.padding);
            let o_value = o_memo.as_ref().map(|x| x.value()).unwrap_or(&self.vec.padding);
            let s_value = if s_hit { s_value } else { s_padding };
            let o_value = if o_hit { o_value } else { o_padding };

            // Compare values.
            if !PartialEq::eq(s_value, o_value) {
                return false;
            }

            i = c_i + 1;
        }

        true
    }
}

impl<'a, T> PartialOrd for SparseSlice<'a, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Prepare common values.
        let s_padding = &self.vec.padding;
        let o_padding = &other.vec.padding;
        let s_start = self.range.start;
        let o_start = self.range.start;
        let s_len = self.range.len();
        let o_len = other.range.len();
        let len = usize::min(s_len, o_len);
        let cmp_len = PartialOrd::partial_cmp(&s_len, &o_len);

        // Prepare loop variables.
        let mut i = 0;
        let mut s_reader = self.sparse_reader();
        let mut o_reader = other.sparse_reader();
        let mut s_memo = None as Option<ElmReader<'_, T>>;
        let mut o_memo = None as Option<ElmReader<'_, T>>;

        // Loop shared part.
        while i < len {
            // Update memos for index.
            let s_index = s_start + i;
            let o_index = o_start + i;
            let s_fresh = s_memo.as_ref().is_some_and(|x| x.index() > s_index);
            let o_fresh = o_memo.as_ref().is_some_and(|x| x.index() > o_index);
            s_memo = if s_fresh { s_memo } else { s_reader.next() };
            o_memo = if o_fresh { o_memo } else { o_reader.next() };

            // Update indexs.
            let s_i = s_memo.as_ref().map(|x| x.index() - s_start).unwrap_or(len);
            let o_i = o_memo.as_ref().map(|x| x.index() - o_start).unwrap_or(len);
            let c_i = usize::min(s_i, o_i);
            let s_hit = c_i == s_i;
            let o_hit = c_i == o_i;

            // Update values.
            let s_value = s_memo.as_ref().map(|x| x.value()).unwrap_or(&self.vec.padding);
            let o_value = o_memo.as_ref().map(|x| x.value()).unwrap_or(&self.vec.padding);
            let s_value = if s_hit { s_value } else { s_padding };
            let o_value = if o_hit { o_value } else { o_padding };

            // Compare values.
            match PartialOrd::partial_cmp(s_value, o_value) {
                Some(Ordering::Equal) => {},
                x => return x,
            }

            i = c_i + 1;
        }

        cmp_len
    }
}