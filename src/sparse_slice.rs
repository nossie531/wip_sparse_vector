//! Provider of [`SparseSlice`].

use crate::common::*;
use crate::iters::*;
use crate::prelude::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Index, Range, RangeBounds};

/// A slice for [`SparseVec`].
///
/// # Examples
///
/// ```
/// # use sparse_vector::prelude::*;
/// let v = SparseVec::from_iter([0, 1, 0, 3, 0]);
/// let s = v.slice(1..4);
/// assert_eq!(s[2], 3);
/// ```
#[repr(C)]
#[must_use]
#[derive(Debug, Eq)]
pub struct SparseSlice<'a, T>
where
    T: PartialEq,
{
    /// Target sparse vector.
    vec: &'a SparseVec<T>,

    /// Slicing range.
    range: Range<usize>,
}

impl<'a, T> SparseSlice<'a, T>
where
    T: PartialEq,
{
    /// Returns `true` if slice is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = v.slice(3..3);
    /// assert!(s.is_empty());
    ///
    /// let s = v.slice(1..4);
    /// assert!(!s.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns slice length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = v.slice(1..4);
    /// assert_eq!(s.len(), 3);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.range.len()
    }

    /// Returns a vector with the same contents of this slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = v.slice(1..4);
    /// assert_eq!(s.to_vec(), vec![2, 3, 4]);
    /// ```
    #[must_use]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        Vec::from_iter(self.iter().cloned())
    }

    /// Returns a slice of specified range.
    ///
    /// # Panics
    ///
    /// Panics in the following cases.
    ///
    /// - Range start and end is reverse order
    /// - Range end is greater than this slice length
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 2, 3, 4, 5, 6]);
    /// let s1 = v.slice(1..5);
    /// let s2 = s1.slice(1..3);
    /// assert_eq!(s2.to_vec(), vec![3, 4]);
    /// ```
    pub fn slice<R>(&self, range: R) -> SparseSlice<'_, T>
    where
        R: RangeBounds<usize>,
    {
        let vec = self.vec;
        let len = self.range.len();
        let range = util::normalize_range(range, len);
        let range = (self.range.start + range.start)..(self.range.start + range.end);
        Self { vec, range }
    }

    /// Returns an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 0, 2, 0, 3]);
    /// let s = v.slice(1..4);
    /// let iter = &mut s.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.vec, self.range.clone())
    }

    /// Returns none padding elements reader.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 2, 0, 4, 5]);
    /// let s = v.slice(1..4);
    /// let iter = &mut s.sparse_reader();
    /// assert_eq!(iter.next(), Some((0, &2)));
    /// assert_eq!(iter.next(), Some((2, &4)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn sparse_reader(&self) -> SparseReader<'_, T> {
        SparseReader::new(self.vec, self.range.clone())
    }

    /// Creates a new instance.
    pub(crate) fn new(vec: &'a SparseVec<T>, range: Range<usize>) -> Self {
        assert!(range.end <= vec.len);
        Self { vec, range }
    }
}

impl<'a, T> Hash for SparseSlice<'a, T>
where
    T: PartialEq + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let len = self.range.len();
        let padding = &self.vec.padding;
        let mut last_index = None;
        let mut last_value = None;
        let mut value_len = 0usize;

        for elm in self.sparse_reader() {
            let next_index = last_index.map_or(0, |x| x + 1);
            let padding_len = elm.0 - next_index;
            let value_changed = padding_len > 0 || Some(elm.1) != last_value;

            if value_changed {
                if value_len > 0 {
                    (last_value.unwrap(), value_len).hash(state);
                }
                if padding_len > 0 {
                    (padding, padding_len).hash(state);
                }
            }

            last_index = Some(elm.0);
            last_value = Some(elm.1);
            value_len = if value_changed { 1 } else { value_len + 1 };
        }

        let padding_len = len - last_index.map_or(0, |x| x + 1);

        if value_len > 0 {
            (last_value.unwrap(), value_len).hash(state);
        }

        if padding_len > 0 {
            (padding, padding_len).hash(state);
        }
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

        // Prepare loop variables.
        let mut i = 0;
        let mut s_reader = self.sparse_reader();
        let mut o_reader = other.sparse_reader();
        let mut s_memo = None as Option<(usize, &T)>;
        let mut o_memo = None as Option<(usize, &T)>;

        // Loop shared part.
        while i < len {
            // Update memos for index.
            let s_fresh = s_memo.as_ref().is_some_and(|x| x.0 >= i);
            let o_fresh = o_memo.as_ref().is_some_and(|x| x.0 >= i);
            s_memo = if s_fresh { s_memo } else { s_reader.next() };
            o_memo = if o_fresh { o_memo } else { o_reader.next() };

            // Update indexs.
            let s_index = s_memo.as_ref().map(|x| x.0).unwrap_or(len);
            let o_index = o_memo.as_ref().map(|x| x.0).unwrap_or(len);
            let c_index = usize::min(s_index, o_index);
            let s_hit = c_index == s_index;
            let o_hit = c_index == o_index;

            // Compare skipped paddings.
            if c_index > i && !PartialEq::eq(s_padding, o_padding) {
                return false;
            }

            // Update values.
            let s_value = s_memo.as_ref().map(|x| x.1).unwrap_or(s_padding);
            let o_value = o_memo.as_ref().map(|x| x.1).unwrap_or(o_padding);
            let s_value = if s_hit { s_value } else { s_padding };
            let o_value = if o_hit { o_value } else { o_padding };

            // Compare values.
            if !PartialEq::eq(s_value, o_value) {
                return false;
            }

            i = c_index + 1;
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
        let s_len = self.range.len();
        let o_len = other.range.len();
        let len = usize::min(s_len, o_len);
        let cmp_len = PartialOrd::partial_cmp(&s_len, &o_len);

        // Prepare loop variables.
        let mut i = 0;
        let mut s_reader = self.sparse_reader();
        let mut o_reader = other.sparse_reader();
        let mut s_memo = None as Option<(usize, &T)>;
        let mut o_memo = None as Option<(usize, &T)>;

        // Loop shared part.
        while i < len {
            // Update memos for index.
            let s_fresh = s_memo.as_ref().is_some_and(|x| x.0 >= i);
            let o_fresh = o_memo.as_ref().is_some_and(|x| x.0 >= i);
            s_memo = if s_fresh { s_memo } else { s_reader.next() };
            o_memo = if o_fresh { o_memo } else { o_reader.next() };

            // Update indexs.
            let s_index = s_memo.as_ref().map(|x| x.0).unwrap_or(len);
            let o_index = o_memo.as_ref().map(|x| x.0).unwrap_or(len);
            let c_index = usize::min(s_index, o_index);
            let s_hit = c_index == s_index;
            let o_hit = c_index == o_index;

            // Compare skipped paddings.
            if c_index > i {
                match PartialOrd::partial_cmp(s_padding, o_padding) {
                    Some(Ordering::Equal) => {}
                    x => return x,
                }
            }

            // Update values.
            let s_value = s_memo.as_ref().map(|x| x.1).unwrap_or(s_padding);
            let o_value = o_memo.as_ref().map(|x| x.1).unwrap_or(o_padding);
            let s_value = if s_hit { s_value } else { s_padding };
            let o_value = if o_hit { o_value } else { o_padding };

            // Compare values.
            match PartialOrd::partial_cmp(s_value, o_value) {
                Some(Ordering::Equal) => {}
                x => return x,
            }

            i = c_index + 1;
        }

        cmp_len
    }
}
