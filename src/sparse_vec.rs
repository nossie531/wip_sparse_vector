use crate::iter::{IntoIter, Iter, SparseReader, SparseWriter};
use crate::values::ValueEditor;
use crate::{ElmReader, SparseVecAll, util};
use pstd::collections::btree_map::BTreeMap;
use std::cmp::Ordering;
use std::ops::{Bound, Deref, DerefMut, Index};

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
    len: usize,

    /// Padding value.
    padding: T,

    /// Padding duplicator.
    filler: fn(&T) -> T,

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

    /// Returns an iterator.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.len, self.padding(), self.map.range(..))
    }

    /// Resutns none padding elements reader.
    pub fn sparse_reader(&self) -> SparseReader<'_, T> {
        SparseReader::new(self.map.iter())
    }

    /// Returns value editor.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not less than vector length.
    #[must_use]
    pub fn edit(&mut self, index: usize) -> ValueEditor<'_, T> {
        assert!(index < self.len());
        let padding = &self.padding;
        let entry = self.map.entry(index);
        ValueEditor::new(padding, entry)
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

    /// Resutns none padding elements writer.
    pub fn sparse_writer(&mut self) -> SparseWriter<'_, T> {
        let padding = &self.padding;
        let cursor = self.map.lower_bound_mut(Bound::Unbounded);
        SparseWriter::new(padding, cursor)
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
    type Target = SparseVecAll<T>;

    fn deref(&self) -> &Self::Target {
        SparseVecAll::from_ref(self)
    }
}

impl<T> DerefMut for SparseVec<T>
where
    T: PartialEq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        SparseVecAll::from_mut(self)
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
        Iter::new(self.len, &self.padding, self.map.range(..))
    }
}

impl<T> Ord for SparseVec<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> PartialEq for SparseVec<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        // Prepare common values.
        let len = self.len;
        let s_padding = &self.padding;
        let o_padding = &other.padding;

        // Prepare loop variables.
        let mut index = 0;
        let mut s_reader = self.sparse_reader();
        let mut o_reader = other.sparse_reader();
        let mut s_memo = None as Option<ElmReader<'_, T>>;
        let mut o_memo = None as Option<ElmReader<'_, T>>;

        // Loop shared part.
        while index < len {
            // Update memos for index.
            let s_fresh = s_memo.as_ref().is_some_and(|x| index < x.index());
            let o_fresh = o_memo.as_ref().is_some_and(|x| index < x.index());
            s_memo = if s_fresh { s_memo } else { s_reader.next() };
            o_memo = if o_fresh { o_memo } else { o_reader.next() };

            // Update indexs.
            let s_index = s_memo.as_ref().map(|x| x.index()).unwrap_or(len);
            let o_index = o_memo.as_ref().map(|x| x.index()).unwrap_or(len);
            let n_index = usize::min(s_index, o_index);
            let s_hit = n_index == s_index;
            let o_hit = n_index == o_index;

            // Update values.
            let s_value = s_memo.as_ref().map(|x| x.value()).unwrap_or(&self.padding);
            let o_value = o_memo.as_ref().map(|x| x.value()).unwrap_or(&self.padding);
            let s_value = if s_hit { s_value } else { s_padding };
            let o_value = if o_hit { o_value } else { o_padding };

            // Compare values.
            match PartialEq::eq(s_value, o_value) {
                true => index = n_index + 1,
                false => return false,
            }
        }

        true
    }
}

impl<T> PartialOrd for SparseVec<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Prepare common values.
        let len = usize::min(self.len, other.len);
        let cmp_len = PartialOrd::partial_cmp(&self.len, &other.len);
        let s_padding = &self.padding;
        let o_padding = &other.padding;

        // Prepare loop variables.
        let mut index = 0;
        let mut s_reader = self.sparse_reader();
        let mut o_reader = other.sparse_reader();
        let mut s_memo = None as Option<ElmReader<'_, T>>;
        let mut o_memo = None as Option<ElmReader<'_, T>>;

        // Loop shared part.
        while index < len {
            // Update memos for index.
            let s_fresh = s_memo.as_ref().is_some_and(|x| index < x.index());
            let o_fresh = o_memo.as_ref().is_some_and(|x| index < x.index());
            s_memo = if s_fresh { s_memo } else { s_reader.next() };
            o_memo = if o_fresh { o_memo } else { o_reader.next() };

            // Update indexs.
            let s_index = s_memo.as_ref().map(|x| x.index()).unwrap_or(len);
            let o_index = o_memo.as_ref().map(|x| x.index()).unwrap_or(len);
            let n_index = usize::min(s_index, o_index);
            let s_hit = n_index == s_index;
            let o_hit = n_index == o_index;

            // Update values.
            let s_value = s_memo.as_ref().map(|x| x.value()).unwrap_or(&self.padding);
            let o_value = o_memo.as_ref().map(|x| x.value()).unwrap_or(&self.padding);
            let s_value = if s_hit { s_value } else { s_padding };
            let o_value = if o_hit { o_value } else { o_padding };

            // Compare values.
            match PartialOrd::partial_cmp(s_value, o_value) {
                Some(Ordering::Equal) => index = n_index + 1,
                x => return x,
            }
        }

        cmp_len
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
