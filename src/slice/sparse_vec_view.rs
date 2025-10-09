use std::cmp::Ordering;
use std::ops::{Bound, Index, RangeBounds};
use crate::prelude::*;
use crate::util;
use crate::slice::SparseVecPart;
use crate::values::ValueEditor;
use crate::iter::{Iter, SparseReader, SparseWriter};

#[repr(transparent)]
#[derive(Debug, Default, Eq, Hash)]
pub struct SparseVecView<T>(SparseVec<T>)
where
    T: PartialEq;

impl<T> SparseVecView<T>
where
    T: PartialEq,
{
    pub fn from_ref(r: &SparseVec<T>) -> &Self {
        unsafe { std::mem::transmute(r) }
    }

    pub fn from_mut(r: &mut SparseVec<T>) -> &mut Self {
        unsafe { std::mem::transmute(r) }
    }
}

impl<T> SparseSlice<T> for SparseVecView<T>
where
    T: PartialEq,
{
    fn len(&self) -> usize {
        self.0.len
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter::new(&self.0, 0..self.0.len)
    }

    fn slice<R>(&self, range: R) -> SparseVecPart<'_, T>
    where 
        R: RangeBounds<usize>,
    {
        let range = util::to_index_range(range, self.0.len());
        SparseVecPart::new(&self.0, range)
    }

    fn sparse_reader(&self) -> SparseReader<'_, T> {
        SparseReader::new(self.0.map.range(..))
    }
}

impl<T> SparseSliceMut<T> for SparseVecView<T>
where 
    T: PartialEq,
{
    fn sparse_writer(&mut self) -> SparseWriter<'_, T> {
        let padding = &self.0.padding;
        let cursor = self.0.map.lower_bound_mut(Bound::Unbounded);
        SparseWriter::new(padding, cursor)
    }

    fn take(&mut self, index: usize) -> Option<T> {
        assert!(index < self.0.len);
        self.0.map.remove(&index)
    }

    fn edit(&mut self, index: usize) -> ValueEditor<'_, T> {
        assert!(index < self.0.len);
        let padding = &self.0.padding;
        let filler = self.0.filler;
        let entry = self.0.map.entry(index);
        ValueEditor::new(padding, filler, entry)
    }
}

impl<T> Index<usize> for SparseVecView<T>
where 
    T: PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.0.len);
        self.0.map.get(&index).unwrap_or(&self.0.padding)
    }
}

impl<'a, T> IntoIterator for &'a SparseVecView<T>
where 
    T: PartialEq,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Ord for SparseVecView<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> PartialEq for SparseVecView<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let slice_x = SparseVecPart::new(&self.0, 0..self.len());
        let slice_y = SparseVecPart::new(&other.0, 0..other.len());
        slice_x.eq(&slice_y)
    }
}

impl<T> PartialOrd for SparseVecView<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let slice_x = SparseVecPart::new(&self.0, 0..self.len());
        let slice_y = SparseVecPart::new(&other.0, 0..other.len());
        slice_x.partial_cmp(&slice_y)
    }
}

// TODO: スライス側にもトレイト実装が必要。
// というより、むしろこっちを本体にしないと…。
