//! Crate's utility.

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
