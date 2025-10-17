use pstd::collections::btree_map::{BTreeMap, CursorMut, Range};

pub type Map<T> = BTreeMap<usize, T>;
pub type MapCursor<'a, T> = CursorMut<'a, usize, T>;
pub type MapRange<'a, T> = Range<'a, usize, T>;
