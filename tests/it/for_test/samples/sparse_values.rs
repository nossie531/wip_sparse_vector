//! Samples for sparse values.

use crate::for_test::builders::ValuesBuilder;

/// Returns specified length sparse values vector.
pub fn sparse_values(len: usize) -> Vec<i32> {
    let builders = ValuesBuilder::new().set_len(len);
    builders.values()
}
