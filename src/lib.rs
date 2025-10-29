//! Vector for sparse values.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._
//!
//! This crate provides a vector like type `SparseVec`.
//! This type is efficient when most elements have same value.

#![allow(clippy::should_implement_trait)]
#![warn(missing_docs)]

pub mod prelude;

pub use iters::*;
pub use sparse_slice::*;
pub use sparse_slice_mut::*;
pub use sparse_vec::*;
pub use value_editor::*;

mod aliases;
mod common;
mod iters;
mod shared_map;
mod padding;
mod sparse_slice;
mod sparse_slice_mut;
mod sparse_vec;
mod value_editor;

pub(crate) use padding::*;
