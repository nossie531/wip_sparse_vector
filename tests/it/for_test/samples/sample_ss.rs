use crate::for_test::SliceContext;
use crate::for_test::builders::SparseSliceBuilder;

pub fn empty() -> SliceContext<i32> {
    let builder = SparseSliceBuilder::new().set_len(0);
    builder.build()
}

pub fn normal() -> SliceContext<i32> {
    let builder = SparseSliceBuilder::new();
    builder.build()
}
