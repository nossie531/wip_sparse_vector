use std::fmt::Debug;

/// Element from sparse vector writer.
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElmWriter<'a, T>
where
    T: PartialEq,
{
    index: usize,
    value: &'a mut T,
}

impl<'a, T> ElmWriter<'a, T>
where
    T: PartialEq,
{
    pub(crate) fn new(index: usize, value: &'a mut T) -> Self {
        Self { index, value }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn value(&self) -> &T {
        self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        self.value
    }
}
