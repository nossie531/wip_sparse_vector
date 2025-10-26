//! Provider of [`SparseSliceMut`].

use crate::ValueEditor;
use crate::common::util;
use crate::iters::*;
use crate::prelude::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::{Index, Range, RangeBounds};

/// A mutable slice for [`SparseVec`].
///
/// # Examples
///
/// ```
/// # use sparse_vector::prelude::*;
/// let v = &mut SparseVec::from_iter([0, 1, 0, 3, 0, 5]);
/// let s = &mut v.slice_mut(1..4);
/// for (_idx, val) in s.sparse_writer() {
///     *val += 1;
/// }
///
/// assert_eq!(v.to_vec(), vec![0, 2, 0, 4, 0, 5]);
/// ```
#[repr(C)]
#[must_use]
#[derive(Debug, Eq)]
pub struct SparseSliceMut<'a, T>
where
    T: PartialEq,
{
    /// Target sparse vector.
    vec: &'a mut SparseVec<T>,

    /// Slicing range.
    range: Range<usize>,
}

impl<'a, T> SparseSliceMut<'a, T>
where
    T: PartialEq,
{
    /// Returns `true` if slice is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = v.slice_mut(3..3);
    /// assert!(s.is_empty());
    ///
    /// let s = v.slice_mut(1..4);
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
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = v.slice_mut(1..4);
    /// assert_eq!(s.len(), 3);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.slice_ref().len()
    }

    /// Returns a vector with the same contents of this slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = v.slice_mut(1..4);
    /// assert_eq!(s.to_vec(), vec![2, 3, 4]);
    /// ```
    #[must_use]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.slice_ref().to_vec()
    }

    /// Returns slice reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = v.slice_mut(1..4);
    /// let s = s.slice_ref();
    /// assert_eq!(s.to_vec(), vec![2, 3, 4]);
    /// ```
    pub fn slice_ref(&self) -> &SparseSlice<'_, T> {
        unsafe { mem::transmute(self) }
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
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5, 6]);
    /// let s1 = v.slice_mut(1..5);
    /// let s2 = s1.slice(1..3);
    /// assert_eq!(s2.to_vec(), vec![3, 4]);
    /// ```
    pub fn slice<R>(&self, range: R) -> SparseSlice<'_, T>
    where
        R: RangeBounds<usize>,
    {
        let len = self.range.len();
        let range = util::normalize_range(range, len);
        self.slice_ref().slice(range)
    }

    /// Returns an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 0, 2, 0, 3]);
    /// let s = v.slice_mut(1..4);
    /// let iter = &mut s.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        self.slice_ref().iter()
    }

    /// Returns none padding elements reader.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 2, 0, 4, 5]);
    /// let s = v.slice_mut(1..4);
    /// let iter = &mut s.sparse_reader();
    /// assert_eq!(iter.next(), Some((0, &2)));
    /// assert_eq!(iter.next(), Some((2, &4)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn sparse_reader(&self) -> SparseReader<'_, T> {
        self.slice_ref().sparse_reader()
    }

    /// Returns none padding elements writer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 0, 3, 0, 5, 0, 7]);
    /// let s = &mut v.slice_mut(1..5);
    /// for (_idx, val) in s.sparse_writer() {
    ///     *val += 1;
    /// }
    ///
    /// assert_eq!(v.to_vec(), vec![1, 0, 4, 0, 6, 0, 7]);
    /// ```
    pub fn sparse_writer(&mut self) -> SparseWriter<'_, T> {
        SparseWriter::new(self.vec, self.range.clone())
    }

    /// Takes the value of index, leaving padding value.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not less than vector length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = &mut v.slice_mut(1..4);
    /// let mut r = s.take(1);
    /// assert_eq!(r, 3);
    /// assert_eq!(v.to_vec(), vec![1, 2, 0, 4, 5]);
    /// ```
    pub fn take(&mut self, index: usize) -> T {
        assert!(index < self.range.len());
        let removed = self.vec.map.remove(&(self.range.start + index));
        removed.unwrap_or(self.vec.padding_val())
    }

    /// Returns value editor.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not less than vector length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = &mut v.slice_mut(1..4);
    /// *s.edit(1) = 42;
    /// assert_eq!(v.to_vec(), vec![1, 2, 42, 4, 5]);
    /// ```
    pub fn edit(&mut self, index: usize) -> ValueEditor<'_, T> {
        assert!(index < self.range.len());
        let padding = &self.vec.padding;
        let entry = self.vec.map.entry(self.range.start + index);
        ValueEditor::new(padding, entry)
    }

    /// Fills `self` with elements by cloning `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = &mut v.slice_mut(1..4);
    /// s.fill(42);
    /// assert_eq!(v.to_vec(), vec![1, 42, 42, 42, 5]);
    /// ```
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.fill_with(|| value.clone());
    }

    /// Fills `self` with elements returned by calling a closure repeatedly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = &mut v.slice_mut(1..4);
    /// s.fill_with(|| 42);
    /// assert_eq!(v.to_vec(), vec![1, 42, 42, 42, 5]);
    /// ```
    pub fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        for i in 0..self.len() {
            let value = f();
            *self.edit(i) = value;
        }
    }

    /// Swaps two elements.
    ///
    /// # Panics
    ///
    /// Panics if `a` or `b` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = &mut SparseVec::from_iter(["a", "b", "c", "d", "e"]);
    /// let s = &mut v.slice_mut(1..4);
    /// s.swap(0, 2);
    /// assert_eq!(v.to_vec(), vec!["a", "d", "c", "b", "e"]);
    /// ```
    pub fn swap(&mut self, x: usize, y: usize) {
        assert!(x < self.len());
        assert!(y < self.len());

        if x != y {
            let xv = self.take(x);
            let yv = self.take(y);

            if &xv != self.vec.padding_ref() {
                *self.edit(y) = xv;
            }

            if &yv != self.vec.padding_ref() {
                *self.edit(x) = yv;
            }
        }
    }

    /// Creates a new instance.
    pub(crate) fn new(vec: &'a mut SparseVec<T>, range: Range<usize>) -> Self {
        assert!(range.end <= vec.len);
        Self { vec, range }
    }
}

impl<'a, T> Hash for SparseSliceMut<'a, T>
where
    T: PartialEq + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.slice_ref().hash(state);
    }
}

impl<'a, T> Index<usize> for SparseSliceMut<'a, T>
where
    T: PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.range.len());
        self.slice_ref().index(index)
    }
}

impl<'a, T> IntoIterator for &'a SparseSliceMut<'a, T>
where
    T: PartialEq,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice_ref().into_iter()
    }
}

impl<'a, T> Ord for SparseSliceMut<'a, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.slice_ref().cmp(other.slice_ref())
    }
}

impl<'a, T> PartialEq for SparseSliceMut<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.slice_ref().eq(other.slice_ref())
    }
}

impl<'a, T> PartialOrd for SparseSliceMut<'a, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.slice_ref().partial_cmp(other.slice_ref())
    }
}
