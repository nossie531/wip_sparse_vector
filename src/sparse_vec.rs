//! Provider of [`SparseVec`].

use crate::ValueEditor;
use crate::aliases::*;
use crate::common::*;
use crate::iters::*;
use crate::prelude::*;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{Index, RangeBounds};

/// A sparse vector.
///
/// This vector has [`padding`] value as elements default value. We can
/// save padding value as vector element value without using memory. So,
/// more percentage of padding values in the vector results in lower
/// memory usage and speedy iteration.
///
/// [`padding`]: Self::padding()
///
/// # Examples
///
/// ```
/// # use sparse_vector::prelude::*;
/// let mut v = SparseVec::new(5);
/// *v.edit(0) = 1;
/// *v.edit(2) = 3;
/// *v.edit(4) = 5;
///
/// assert_eq!(v.to_vec(), vec![1, 0, 3, 0, 5]);
///
/// for (_i, v) in v.sparse_writer() {
///     *v += 1;
/// }
///
/// assert_eq!(v.to_vec(), vec![2, 0, 4, 0, 6]);
/// ```
#[derive(Clone, Debug, Eq)]
pub struct SparseVec<T>
where
    T: PartialEq,
{
    /// Vector length.
    pub(crate) len: usize,

    /// Padding value.
    pub(crate) padding: T,

    /// Padding value duplicator.
    pub(crate) filler: fn(&T) -> T,

    /// None padding elements map.
    pub(crate) map: Map<T>,
}

impl<T> SparseVec<T>
where
    T: PartialEq,
{
    /// Creates a new instance with default padding value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::<i32>::new(10);
    /// assert_eq!(v.len(), 10);
    /// assert_eq!(v.padding(), &0);
    /// assert_eq!(v[5], 0);
    /// ```
    #[must_use]
    pub fn new(len: usize) -> Self
    where
        T: Default,
    {
        Self {
            len,
            padding: T::default(),
            filler: util::default_like_clone,
            map: Map::new(),
        }
    }

    /// Creates a new instance with padding value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::<i32>::with_padding(10, 42);
    /// assert_eq!(v.len(), 10);
    /// assert_eq!(v.padding(), &42);
    /// assert_eq!(v[5], 42);
    /// ```
    #[must_use]
    pub fn with_padding(len: usize, padding: T) -> Self
    where
        T: Clone,
    {
        Self {
            len,
            padding,
            filler: T::clone,
            map: Map::new(),
        }
    }

    /// Returns `true` if this contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::<i32>::new(0);
    /// assert!(v.is_empty());
    ///
    /// v.push(1);
    /// assert!(!v.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if this contains no none padding elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::<i32>::new(10);
    /// assert!(v.is_all_padding());
    ///
    /// *v.edit(5) = 1;
    /// assert!(!v.is_all_padding());
    /// ```
    #[must_use]
    pub fn is_all_padding(&self) -> bool {
        self.nnp() == 0
    }

    /// Returns the number of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::<i32>::new(10);
    /// assert_eq!(v.len(), 10);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns NNP (the Number of None Padding elements).
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::<i32>::new(10);
    /// assert_eq!(v.nnp(), 0);
    ///
    /// *v.edit(5) = 1;
    /// assert_eq!(v.nnp(), 1);
    /// ```
    #[must_use]
    pub fn nnp(&self) -> usize {
        self.map.len()
    }

    /// Returns the padding reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::<i32>::with_padding(10, 42);
    /// assert_eq!(v.padding(), &42);
    /// ```
    #[must_use]
    pub fn padding(&self) -> &T {
        &self.padding
    }

    /// Returns cloned padding value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::<i32>::with_padding(10, 42);
    /// assert_eq!(v.clone_padding(), 42);
    /// ```
    #[must_use]
    pub fn clone_padding(&self) -> T {
        (self.filler)(&self.padding)
    }

    /// Returns a vector with the same contents of this sparse vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 2, 3]);
    /// assert_eq!(v.to_vec(), vec![1, 2, 3]);
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
    /// - Range end is greater than this vector length
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let s = v.slice(1..4);
    /// assert_eq!(s.to_vec(), vec![2, 3, 4]);
    /// ```
    pub fn slice<R>(&self, range: R) -> SparseSlice<'_, T>
    where
        R: RangeBounds<usize>,
    {
        let range = util::normalize_range(range, self.len);
        SparseSlice::new(self, range)
    }

    /// Returns an iterator over this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 2, 3]);
    /// let iter = &mut v.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self, 0..self.len)
    }

    /// Returns none padding elements reader.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let v = SparseVec::from_iter([1, 0, 3, 0, 5]);
    /// let iter = &mut v.sparse_reader();
    /// assert_eq!(iter.next(), Some((0, &1)));
    /// assert_eq!(iter.next(), Some((2, &3)));
    /// assert_eq!(iter.next(), Some((4, &5)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn sparse_reader(&self) -> SparseReader<'_, T> {
        SparseReader::new(self, util::normalize_range(.., self.len))
    }

    /// Sets vector length.
    ///
    /// If specified value is less than this vector current length,
    /// this vector will become shorter. If specifed value is greater
    /// than this vector current length, this vector will become longer
    /// and new elements are filled by padding value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2, 3]);
    /// v.set_len(5);
    /// assert_eq!(v.to_vec(), vec![1, 2, 3, 0, 0]);
    /// ```
    pub fn set_len(&mut self, value: usize) {
        self.len = value;
        while let Some(last) = self.map.last_entry() {
            if *last.key() < value {
                return;
            }

            let _ = last.remove();
        }
    }

    /// Returns a mutable slice of specified range.
    ///
    /// # Panics
    ///
    /// Panics in the following cases.
    ///
    /// - Range start and end is reverse order
    /// - Range end is greater than this vector length
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// let mut s = v.slice_mut(1..4);
    /// *s.edit(0) += 10;
    /// *s.edit(1) += 10;
    /// *s.edit(2) += 10;
    /// assert_eq!(v.to_vec(), vec![1, 12, 13, 14, 5]);
    /// ```
    pub fn slice_mut<R>(&mut self, range: R) -> SparseSliceMut<'_, T>
    where
        R: RangeBounds<usize>,
    {
        let range = util::normalize_range(range, self.len);
        SparseSliceMut::new(self, range)
    }

    /// Returns a none padding elements writer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// #
    /// let mut v = SparseVec::from_iter([1, 0, 3, 0, 5]);
    /// let mut iter = v.sparse_writer();
    /// while let Some(mut item) = iter.next() {
    ///     *item.1 += 1;
    /// }
    ///
    /// drop(iter);
    ///
    /// assert_eq!(v.to_vec(), vec![2, 0, 4, 0, 6]);
    /// ```
    pub fn sparse_writer(&mut self) -> SparseWriter<'_, T> {
        SparseWriter::new(self, 0..self.len())
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
    /// let mut v = SparseVec::from_iter([1, 2, 3]);
    /// let mut r = v.take(1);
    /// assert_eq!(r, 2);
    /// assert_eq!(v.to_vec(), vec![1, 0, 3]);
    /// ```
    pub fn take(&mut self, index: usize) -> T {
        assert!(index < self.len);
        self.slice_mut(..).take(index)
    }

    /// Returns a value editor.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not less than vector length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2, 3]);
    /// *v.edit(1) = 42;
    /// assert_eq!(v.to_vec(), vec![1, 42, 3]);
    /// ```
    pub fn edit(&mut self, index: usize) -> ValueEditor<'_, T> {
        assert!(index < self.len);
        let padding = &self.padding;
        let filler = self.filler;
        let entry = self.map.entry(index);
        ValueEditor::new(padding, filler, entry)
    }

    /// Removes the last element from and returns it, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2, 3]);
    /// let r = v.pop();
    /// assert_eq!(r, Some(3));
    /// assert_eq!(v.to_vec(), vec![1, 2]);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let last_index = self.len - 1;
        let last_from_map = self.map.remove(&last_index);
        let ret = last_from_map.unwrap_or_else(|| self.clone_padding());
        self.len -= 1;
        Some(ret)
    }

    /// Appends new last element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2]);
    /// v.push(3);
    /// assert_eq!(v.to_vec(), vec![1, 2, 3]);
    /// ```
    pub fn push(&mut self, value: T) {
        if self.padding != value {
            let new_index = self.len;
            self.map.insert(new_index, value);
        }

        self.len += 1;
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
    /// let mut v = SparseVec::from_iter(["a", "b", "c", "d", "e"]);
    /// v.swap(2, 4);
    /// assert_eq!(v.to_vec(), vec!["a", "b", "e", "d", "c"]);
    /// ```
    pub fn swap(&mut self, x: usize, y: usize) {
        assert!(x < self.len());
        assert!(y < self.len());
        self.slice_mut(..).swap(x, y);
    }

    /// Clears the vector, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2, 3]);
    /// v.clear();
    /// assert!(v.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.len = 0;
        self.map.clear();
    }

    /// Fills `self` with elements by cloning `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2, 3]);
    /// v.fill(42);
    /// assert_eq!(v.to_vec(), vec![42, 42, 42]);
    /// ```
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.slice_mut(..).fill(value);
    }

    /// Fills `self` with elements returned by calling a closure repeatedly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2, 3]);
    /// v.fill_with(|| 42);
    /// assert_eq!(v.to_vec(), vec![42, 42, 42]);
    /// ```
    pub fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T,
    {
        self.slice_mut(..).fill_with(f);
    }

    /// Replace values in specified range to iterator values.
    ///
    /// # Panics
    ///
    /// Panics in the following cases.
    ///
    /// - Range start and end is reverse order
    /// - Range end is greater than this vector length
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped
    /// (due to [`mem::forget`], for example), only action performed is
    /// just setting the values within the `range` to padding values.
    ///
    /// [`mem::forget`]: std::mem::forget
    ///
    /// # Examples
    ///
    /// ```
    /// # use sparse_vector::prelude::*;
    /// let mut v = SparseVec::from_iter([1, 2, 3, 4, 5]);
    /// v.splice(1..3, [42, 43, 44]);
    /// assert_eq!(v.to_vec(), vec![1, 42, 43, 44, 4, 5]);
    /// ```
    pub fn splice<R, I>(
        &mut self,
        range: R,
        replace_with: I,
    ) -> Splice<'_, <I as IntoIterator>::IntoIter>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        let range = util::normalize_range(range, self.len);
        Splice::new(self, range, replace_with.into_iter())
    }
}

impl<T> Default for SparseVec<T>
where
    T: PartialEq + Default,
{
    fn default() -> Self {
        Self::new(0)
    }
}

impl<T> Extend<T> for SparseVec<T>
where
    T: PartialEq,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.len += 1;
            if item != self.padding {
                self.map.insert(self.len - 1, item);
            }
        }
    }
}

impl<'a, T> Extend<&'a T> for SparseVec<T>
where
    T: PartialEq + Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.extend(iter.into_iter().copied());
    }
}

impl<T, const N: usize> From<[T; N]> for SparseVec<T>
where
    T: PartialEq + Default,
{
    fn from(value: [T; N]) -> Self {
        Self::from_iter(value)
    }
}

impl<T> From<Vec<T>> for SparseVec<T>
where
    T: PartialEq + Default,
{
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value)
    }
}

impl<T> FromIterator<T> for SparseVec<T>
where
    T: PartialEq + Default,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut ret = SparseVec::default();
        for (i, item) in iter.into_iter().enumerate() {
            ret.len += 1;
            if item != T::default() {
                ret.map.insert(i, item);
            }
        }

        ret
    }
}

impl<T> Hash for SparseVec<T>
where
    T: PartialEq + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.slice(..).hash(state);
    }
}

impl<T> Index<usize> for SparseVec<T>
where
    T: PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len);
        self.map.get(&index).unwrap_or(&self.padding)
    }
}

impl<T> IntoIterator for SparseVec<T>
where
    T: PartialEq,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a SparseVec<T>
where
    T: PartialEq,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Ord for SparseVec<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.slice(..).cmp(&other.slice(..))
    }
}

impl<T> PartialEq for SparseVec<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.slice(..).eq(&other.slice(..))
    }
}

impl<T> PartialOrd for SparseVec<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.slice(..).partial_cmp(&other.slice(..))
    }
}

impl<T> From<SparseVec<T>> for Vec<T>
where
    T: PartialEq,
{
    fn from(value: SparseVec<T>) -> Self {
        Vec::from_iter(value)
    }
}
