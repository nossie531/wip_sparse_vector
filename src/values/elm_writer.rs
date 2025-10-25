use std::fmt::Debug;

/// Element writer.
/// 
/// This type is created by the [`next`] method on
/// [`SparseWriter`] (provided by the [`Iterator`] trait).
/// See its documentation for more.
/// 
/// [`next`]: crate::iters::SparseWriter::next
/// [`SparseWriter`]: crate::iters::SparseWriter
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElmWriter<'a, T>
where
    T: PartialEq,
{
    /// Element index.
    index: usize,

    /// Element value mutable reference.
    value: &'a mut T,
}

impl<'a, T> ElmWriter<'a, T>
where
    T: PartialEq,
{
    /// Creates a new instance.
    pub(crate) fn new(index: usize, value: &'a mut T) -> Self {
        Self { index, value }
    }

    /// Returns element index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns element value reference.
    pub fn value(&self) -> &T {
        self.value
    }

    /// Returns element value mutable reference.
    pub fn value_mut(&mut self) -> &mut T {
        self.value
    }
}
