//! Type aliases.

use crate::shared_map::*;

/// Map for sparse vector.
pub type Map<T> = SharedMap<usize, T>;

/// Map range iterator for sparse vector.
pub type MapRange<'a, T> = Range<'a, usize, T>;

/// Map range mutable iterator for sparse vector.
pub type MapRangeMut<'a, T> = RangeMut<'a, usize, T>;
