use crate::util;
use crate::values::{ElmWriter, ValueCursor};
use only_one::prelude::*;
use pstd::collections::btree_map::CursorMut;
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[must_use]
pub struct SparseWriter<'a, T>
where
    T: PartialEq,
{
    offset: usize,
    padding: One<&'a T>,
    cursor: One<CursorMut<'a, usize, T>>,
}

impl<'a, T> SparseWriter<'a, T>
where
    T: PartialEq,
{
    pub fn next(&mut self) -> Option<ElmWriter<'a, '_, T>> {
        let elm = self.cursor.next()?;
        let index = *elm.0 - self.offset;
        let padding = &*self.padding;
        let cursor = &mut self.cursor;
        Some(ElmWriter::new(index, ValueCursor::new(padding, cursor)))
    }

    pub(crate) fn new(offset: usize, padding: &'a T, cursor: CursorMut<'a, usize, T>) -> Self {
        Self {
            offset,
            padding: One::new(padding),
            cursor: One::new(cursor),
        }
    }
}

impl<T> Default for SparseWriter<'_, T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            offset: Default::default(),
            padding: Default::default(),
            cursor: Default::default(),
        }
    }
}

/// None derive implementation.
///
/// # TODO for future
///
/// Currently [`CursorMut`] of [`pstd`] does not implement [`Debug`].<br/>
/// Therefore we are not using `derive` attribute at [`Debug`].
impl<T> Debug for SparseWriter<'_, T>
where
    T: Debug + PartialEq,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let type_name = util::name_of_type!(SparseWriter<'_, T>);
        f.debug_struct(type_name)
            .field(util::name_of!(offset in Self), &self.offset)
            .field(util::name_of!(padding in Self), &self.padding)
            .finish_non_exhaustive()
    }
}
