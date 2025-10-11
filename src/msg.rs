//! Messages for this crate.

/// `must_use` attribute message for iterator.
macro_rules! iter_must_use {
    () => {
        "iterators are lazy and do nothing unless consumed"
    };
}

pub(crate) use iter_must_use;

/// Message for cases range end is greater than collection length.
pub fn range_end_gt_len(re: usize, len: usize) -> String {
    format!("Range end {re} is greater than length {len}.")
}

/// Message for cases range start is greater than range end.
pub fn range_order_rev(start: usize, end: usize) -> String {
    format!("Range start {start} is greater than range end {end}.")
}