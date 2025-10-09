//! Crate's utility.

use std::ops::{Bound, Range, RangeBounds};

macro_rules! name_of {
    ($n:ident in $t:ty) => {
        ::nameof::name_of!($n in $t)
    };
}

macro_rules! name_of_type {
    ($t:ty) => {
        ::nameof::name_of_type!($t).split("<").next().unwrap()
    };
}

pub(crate) use name_of;
pub(crate) use name_of_type;

/// Call [`Default::default`] on `T`.
///
/// This function mimics [`Clone::clone`] method signature.
/// Therefore we can substitute [`Clone::clone`] with this function.
pub fn default_like_clone<T: Default>(_x: &T) -> T {
    T::default()
}

/// Normalize range for index.
pub fn to_index_range<R: RangeBounds<usize>>(range: R, len: usize) -> Range<usize> {
    let s = match range.start_bound() {
        Bound::Included(x) => usize::min(len, *x),
        Bound::Excluded(x) => usize::min(len, *x - 1),
        Bound::Unbounded => 0,
    };
    let e = match range.end_bound() {
        Bound::Included(x) => usize::min(len, *x + 1),
        Bound::Excluded(x) => usize::min(len, *x),
        Bound::Unbounded => len,
    };

    s..e
}
