use crate::values::ValueCursor;
use std::fmt::Debug;

/// Element from sparse vector writer.
#[derive(Debug)]
pub struct ElmWriter<'m: 'a, 'a, T>
where
    T: PartialEq,
{
    index: usize,
    cursor: ValueCursor<'m, 'a, T>,
}

impl<'m: 'a, 'a, T> ElmWriter<'m, 'a, T>
where
    T: PartialEq,
{
    pub(crate) fn new(index: usize, cursor: ValueCursor<'m, 'a, T>) -> Self {
        Self { index, cursor }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn value(&self) -> &T {
        self.cursor.value()
    }

    pub fn value_mut(&mut self) -> &mut T {
        self.cursor.value_mut()
    }
}
