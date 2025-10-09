use crate::iter::{IntoIter, Iter};
use crate::{util, SparseVecPart, SparseVecPartMut, SparseVecView};
use pstd::collections::btree_map::BTreeMap;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut, Index, RangeBounds};

/// Sparse vector.
///
/// This vector has [`padding`] value as elements default value. We can
/// save padding value as vector element value without using memory. So,
/// more percentage of padding values in the vector results in lower
/// memory usage and speedy iteration.
///
/// [`padding`]: Self::padding()
#[derive(Clone, Debug, Eq, Hash)]
pub struct SparseVec<T>
where
    T: PartialEq,
{
    /// Vector length.
    pub(crate) len: usize,

    /// Padding value.
    pub(crate) padding: T,

    /// Padding duplicator.
    pub(crate) filler: fn(&T) -> T,

    /// None padding elements map.
    pub(crate) map: BTreeMap<usize, T>,
}

impl<T> SparseVec<T>
where
    T: PartialEq,
{
    /// Creates a new instance with default padding value.
    #[must_use]
    pub fn new(len: usize) -> Self
    where
        T: Default,
    {
        Self {
            len,
            padding: T::default(),
            filler: util::default_like_clone,
            map: BTreeMap::new(),
        }
    }

    /// Creates a new instance with padding value.
    #[must_use]
    pub fn with_padding(len: usize, padding: T) -> Self
    where
        T: Clone,
    {
        Self {
            len,
            padding: padding,
            filler: T::clone,
            map: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn is_all_padding(&self) -> bool {
        self.nnp() == 0
    }

    /// Returns the number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns nnp (the Number of None Padding).
    #[must_use]
    pub fn nnp(&self) -> usize {
        self.map.len()
    }

    /// Returns padding value.
    #[must_use]
    pub fn padding(&self) -> &T {
        &self.padding
    }

    #[must_use]
    pub fn slice<R>(&self, range: R) -> SparseVecPart<'_, T>
    where 
        R: RangeBounds<usize>
    {
        let range = util::to_index_range(range, self.len);
        SparseVecPart::new(self, range)
    }

    #[must_use]
    pub fn slice_mut<R>(&mut self, range: R) -> SparseVecPartMut<'_, T>
    where 
        R: RangeBounds<usize>
    {
        let range = util::to_index_range(range, self.len);
        SparseVecPartMut::new(self, range)
    }

    /// Sets vector length.
    ///
    /// If specified value is less than this vector current length,
    /// this vector will become shorter. If specifed value is greater
    /// than this vector current length, this vector will become longer
    /// and new elements are filled by padding value.
    pub fn set_len(&mut self, value: usize) {
        self.len = value;
        while let Some(last) = self.map.last_entry() {
            if *last.key() < value {
                return;
            }

            let _ = last.remove();
        }
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

impl<T> Deref for SparseVec<T>
where
    T: PartialEq,
{
    type Target = SparseVecView<T>;

    fn deref(&self) -> &Self::Target {
        SparseVecView::from_ref(self)
    }
}

impl<T> DerefMut for SparseVec<T>
where
    T: PartialEq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        SparseVecView::from_mut(self)
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
        for item in iter.into_iter().copied() {
            self.len += 1;
            if item != self.padding {
                self.map.insert(self.len - 1, item);
            }
        }
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
            if item != T::default() {
                ret.map.insert(i, item);
                ret.len += 1;
            }
        }

        ret
    }
}

impl<T> Index<usize> for SparseVec<T>
where
    T: PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len);
        self.deref().index(index)
    }
}

impl<T> IntoIterator for SparseVec<T>
where
    T: PartialEq,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.len, self.padding, self.filler, self.map.into_iter())
    }
}

impl<'a, T> IntoIterator for &'a SparseVec<T>
where
    T: PartialEq,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref().into_iter()
    }
}

impl<T> Ord for SparseVec<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

impl<T> PartialEq for SparseVec<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<T> PartialOrd for SparseVec<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deref().partial_cmp(other.deref())
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
