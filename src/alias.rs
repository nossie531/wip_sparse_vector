use pstd::collections::btree_map::{BTreeMap, Range, RangeMut};

pub type Map<T> = BTreeMap<usize, T>;
pub type MapRange<'a, T> = Range<'a, usize, T>;
pub type MapRangeMut<'a, T> = RangeMut<'a, usize, T>;
