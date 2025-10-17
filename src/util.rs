//! Crate's utility.

use crate::msg;
use std::ops::{Bound, Range, RangeBounds};
use std::panic::panic_any;

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
///
/// # Panics
///
/// Panics in the following cases.
///
/// - Range start and end is reverse order
/// - Range end is greater than this vector length
#[track_caller]
pub fn normalize_range<R: RangeBounds<usize>>(range: R, len: usize) -> Range<usize> {
    let s = match range.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x + 1,
        Bound::Unbounded => 0,
    };
    let e = match range.end_bound() {
        Bound::Included(x) => *x + 1,
        Bound::Excluded(x) => *x,
        Bound::Unbounded => len,
    };

    if s > e {
        panic_any(msg::range_order_rev(s, e))
    }

    if e > len {
        panic_any(msg::range_end_gt_len(e, len));
    }

    s..e
}
