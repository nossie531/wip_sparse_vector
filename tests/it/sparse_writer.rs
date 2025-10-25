use crate::for_test::builders::*;
use crate::for_test::samples::*;
use permute::permutations_of;
use sparse_vector::SparseWriter;
use std::mem;

#[test]
fn default() {
    let result = SparseWriter::<i32>::default();
    assert_eq!(result.size_hint(), (0, Some(0)));
    assert_eq!(result.count(), 0);
}

#[test]
fn drop() {
    // Arrange vec.
    let builder = SparseVecBuilder::new();
    let vec = &mut builder.build();

    // Arrange writer.
    let mut target = vec.sparse_writer();
    let index = builder.npad_indexs().iter().count() / 2;
    let elm = &mut target.nth(index).unwrap();
    *elm.value_mut() = builder.padding();

    // Act.
    mem::drop(target);

    // Assert.
    assert_eq!(vec.nnp(), builder.nnp() - 1);
}

#[test]
fn next() {
    with_default();
    with_empty();
    with_overrun();
    with_normal();
    with_slice();

    fn with_default() {
        let target = &mut SparseWriter::<i32>::default();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_empty() {
        let vec = &mut SparseVecSample::default();
        let target = &mut vec.sparse_writer();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let target = &mut vec.sparse_writer();
        target.nth(builder.nnp() - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let indexs = builder.npad_indexs();
        let index_pos = indexs.len() / 2;
        let target = &mut vec.sparse_writer();
        target.nth(index_pos - 1);

        // Act.
        let result = target.next();

        // Assert.
        let lhs_idx = result.as_ref().unwrap().index();
        let lhs_val = *result.as_ref().unwrap().value();
        let rhs_idx = *indexs.iter().nth(index_pos).unwrap();
        let rhs_val = builder.values()[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }

    fn with_slice() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let slice = &mut context.fetch_mut();
        let target = &mut slice.sparse_writer();

        // Act.
        let result = target.next();

        // Assert.
        let indexs = builder.npad_indexs();
        let lhs_idx = result.as_ref().unwrap().index();
        let lhs_val = *result.as_ref().unwrap().value();
        let rhs_idx = *indexs.first().unwrap();
        let rhs_val = builder.slice_values()[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }
}

#[test]
fn size_hint() {
    with_default();
    with_normal();

    fn with_default() {
        let target = SparseWriter::<i32>::default();
        let result = target.size_hint();
        assert_eq!(result, (0, Some(0)))
    }

    fn with_normal() {
        for mut values in permutations_of(&[5, 10, 15]) {
            // prepare test parameters.
            let nnp_len = *values.next().unwrap();
            let side_len = *values.next().unwrap();
            let slice_len = *values.next().unwrap();
            let vec_len = slice_len + side_len;
            let range = range_for(vec_len).with_len(slice_len);

            // Arrange.
            let builder = SparseVecBuilder::new().set_len(vec_len).set_nnp(nnp_len);
            let vec = &mut builder.build();
            let slice = &mut vec.slice_mut(range);
            let target = slice.sparse_writer();

            // Act.
            let result = target.size_hint();

            // Assert.
            assert_eq!(result.0, nnp_len.saturating_sub(side_len));
            assert_eq!(result.1, Some(usize::min(nnp_len, slice_len)));
        }
    }
}

#[test]
fn next_back() {
    with_default();
    with_empty();
    with_overrun();
    with_normal();
    with_slice();

    fn with_default() {
        let target = &mut SparseWriter::<i32>::default();
        let result = target.next_back();
        assert_eq!(result, None);
    }

    fn with_empty() {
        let vec = &mut SparseVecSample::default();
        let target = &mut vec.sparse_writer();
        let result = target.next_back();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let target = &mut vec.sparse_writer();
        target.nth_back(builder.nnp() - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let indexs = builder.npad_indexs();
        let index_pos = indexs.len() / 2;
        let back_len = indexs.len() - index_pos - 1;
        let target = &mut vec.sparse_writer();
        target.nth_back(back_len - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        let lhs_idx = result.as_ref().unwrap().index();
        let lhs_val = *result.as_ref().unwrap().value();
        let rhs_idx = *indexs.iter().nth(index_pos).unwrap();
        let rhs_val = builder.values()[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }

    fn with_slice() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let slice = &mut context.fetch_mut();
        let target = &mut slice.sparse_writer();

        // Act.
        let result = target.next_back();

        // Assert.
        let indexs = builder.npad_indexs();
        let lhs_idx = result.as_ref().unwrap().index();
        let lhs_val = *result.as_ref().unwrap().value();
        let rhs_idx = *indexs.last().unwrap();
        let rhs_val = builder.slice_values()[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }
}
