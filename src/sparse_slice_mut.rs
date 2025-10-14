use crate::loops::{Iter, SparseReader, SparseWriter};
use crate::prelude::*;
use crate::values::ValueEditor;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::{Bound, Index, Range, RangeBounds};

#[repr(C)]
#[must_use]
#[derive(Debug, Eq)]
pub struct SparseSliceMut<'a, T>
where
    T: PartialEq,
{
    vec: &'a mut SparseVec<T>,
    range: Range<usize>,
}

impl<'a, T> SparseSliceMut<'a, T>
where
    T: PartialEq,
{
    /// Returns `true` if slice is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns slice length.
    pub fn len(&self) -> usize {
        self.slice_ref().len()
    }

    /// Returns slice reference.
    pub fn slice_ref(&self) -> &SparseSlice<'_, T> {
        unsafe { mem::transmute(self) }
    }

    /// Returns an iterator.
    pub fn iter(&self) -> crate::Iter<'_, T> {
        self.slice_ref().iter()
    }

    /// Returns none padding elements reader.
    pub fn sparse_reader(&self) -> SparseReader<'_, T> {
        self.slice_ref().sparse_reader()
    }

    /// Copies `self` into a new [`Vec`].
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.slice_ref().to_vec()
    }

    /// Slice this slice.
    pub fn slice<R>(&self, range: R) -> SparseSlice<'_, T>
    where
        R: RangeBounds<usize>,
    {
        self.slice_ref().slice(range)
    }

    /// Returns none padding elements writer.
    pub fn sparse_writer(&mut self) -> SparseWriter<'_, T> {
        let padding = &self.vec.padding;
        let cursor = self
            .vec
            .map
            .lower_bound_mut(Bound::Included(&self.range.start));
        SparseWriter::new(padding, cursor)
    }

    /// Takes the value of index, leaving padding value.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not less than vector length.
    pub fn take(&mut self, index: usize) -> T {
        assert!(index + self.range.start < self.vec.len);
        let removed = self.vec.map.remove(&(index + self.range.start));
        removed.unwrap_or((self.vec.filler)(&self.vec.padding))
    }

    /// Returns value editor.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not less than vector length.
    pub fn edit(&mut self, index: usize) -> ValueEditor<'_, T> {
        assert!(index + self.range.start < self.vec.len);
        let padding = &self.vec.padding;
        let filler = self.vec.filler;
        let entry = self.vec.map.entry(index + self.range.start);
        ValueEditor::new(padding, filler, entry)
    }

    /// Fills `self` with elements by cloning `value`.
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.fill_with(|| value.clone());
    }

    /// Fills `self` with elements returned by calling a closure repeatedly.
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
    pub fn swap(&mut self, x: usize, y: usize) {
        assert!(x < self.len());
        assert!(y < self.len());

        if x == y {
            return;
        }

        let xv = self.take(x);
        let yv = self.take(y);
        let x_is_padding = xv == self.vec.padding;
        let y_is_padding = yv == self.vec.padding;
        match (x_is_padding, y_is_padding) {
            (true, true) => {}
            (true, false) => {
                *self.edit(x) = yv;
            }
            (false, true) => {
                *self.edit(y) = xv;
            }
            (false, false) => {
                *self.edit(x) = yv;
                *self.edit(y) = xv;
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
