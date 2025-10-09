//! Messages for this crate.

/// `must_use` attribute message for iterator.
macro_rules! iter_must_use {
    () => {
        "iterators are lazy and do nothing unless consumed"
    };
}

pub(crate) use iter_must_use;
