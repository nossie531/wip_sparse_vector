use crate::util;
use crate::values::ElmWriter;
use only_one::One;
use pstd::collections::btree_map::CursorMut;
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[must_use]
pub struct SparseWriter<'a, T>
where
    T: PartialEq,
{
    padding: One<&'a T>,
    cursor: One<CursorMut<'a, usize, T>>,
}

impl<'a, T> SparseWriter<'a, T>
where
    T: PartialEq,
{
    pub fn next(&mut self) -> Option<ElmWriter<'_, 'a, T>> {
        let padding = self.padding;
        let cursor = &mut self.cursor;
        cursor.next()?;
        Some(ElmWriter::new(&padding, cursor))
    }

    pub(crate) fn new(padding: &'a T, cursor: CursorMut<'a, usize, T>) -> Self {
        Self {
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
            .field(util::name_of!(padding in Self), &self.padding)
            .finish_non_exhaustive()
    }
}
