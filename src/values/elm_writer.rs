use crate::util;
use pstd::collections::btree_map::CursorMut;
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Element from sparse vector writer.
pub struct ElmWriter<'a, 'b: 'a, T>
where
    T: PartialEq,
{
    padding: &'a T,
    cursor: &'a mut CursorMut<'b, usize, T>,
}

impl<'a, 'b: 'a, T> ElmWriter<'a, 'b, T>
where
    T: PartialEq,
{
    pub(crate) fn new(padding: &'a T, cursor: &'a mut CursorMut<'b, usize, T>) -> Self {
        Self { padding, cursor }
    }

    pub fn index(&self) -> usize {
        *self.cursor.peek_prev().unwrap().0
    }

    pub fn value(&self) -> &T {
        self.cursor.peek_prev().unwrap().1
    }

    pub fn value_mut(&mut self) -> &mut T {
        self.cursor.peek_prev().unwrap().1
    }
}

impl<T> Drop for ElmWriter<'_, '_, T>
where
    T: PartialEq,
{
    fn drop(&mut self) {
        if self.value() == self.padding {
            self.cursor.remove_prev();
        }
    }
}

/// None derive implementation.
///
/// # TODO for future
///
/// Currently [`CursorMut`] of [`pstd`] does not implement [`Debug`].<br/>
/// Therefore we are not using `derive` attribute at [`Debug`].
impl<T> Debug for ElmWriter<'_, '_, T>
where
    T: Debug + PartialEq,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let type_name = util::name_of_type!(ElmWriter<'_, '_, T>);
        f.debug_struct(type_name)
            .field(util::name_of!(padding in Self), &self.padding)
            .finish_non_exhaustive()
    }
}
