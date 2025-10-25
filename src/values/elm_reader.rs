//! Provider of [`ElmReader`].

/// Element reader.
///
/// This type is created by the [`next`] method on
/// [`SparseReader`] (provided by the [`Iterator`] trait).
/// See its documentation for more.
///
/// [`next`]: crate::iters::SparseReader::next
/// [`SparseReader`]: crate::iters::SparseReader
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElmReader<'a, T>
where
    T: PartialEq,
{
    /// Element index.
    index: usize,

    /// Element value reference.
    value: &'a T,
}

impl<'a, T> ElmReader<'a, T>
where
    T: PartialEq,
{
    /// Creates a new instance.
    pub(crate) fn new(index: usize, value: &'a T) -> Self {
        Self { index, value }
    }

    /// Returns element index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns element value reference.
    pub fn value(&self) -> &'a T {
        self.value
    }
}
