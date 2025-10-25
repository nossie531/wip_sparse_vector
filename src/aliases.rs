//! Type aliases.

use pstd::collections::btree_map::{BTreeMap, Range, RangeMut};

/// Map for sparse vector.
pub type Map<T> = BTreeMap<usize, T>;

/// Map range iterator for sparse vector.
pub type MapRange<'a, T> = Range<'a, usize, T>;

/// Map range mutable iterator for sparse vector.
pub type MapRangeMut<'a, T> = RangeMut<'a, usize, T>;
