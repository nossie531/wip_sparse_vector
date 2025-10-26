use crate::for_test::builders::*;
use crate::for_test::samples::*;
use permute::permutations_of;
use sparse_vector::SparseReader;

#[test]
fn clone() {
    let vec = SparseVecSample::normal();
    let target = vec.sparse_reader();
    let result = target.clone();
    assert!(result.eq(vec.sparse_reader()));
}

#[test]
fn default() {
    let result = SparseReader::<i32>::default();
    assert_eq!(result.size_hint(), (0, Some(0)));
    assert_eq!(result.count(), 0);
}

#[test]
fn next() {
    with_default();
    with_empty();
    with_overrun();
    with_normal();
    with_slice();

    fn with_default() {
        let target = &mut SparseReader::<i32>::default();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_empty() {
        let vec = SparseVecSample::default();
        let target = &mut vec.sparse_reader();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = SparseVecSample::normal();
        let target = &mut vec.sparse_reader();
        target.nth(vec.nnp() - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = builder.build();
        let indexs = builder.npad_indexs();
        let index_pos = indexs.len() / 2;
        let target = &mut vec.sparse_reader();
        target.nth(index_pos - 1);

        // Act.
        let result = target.next();

        // Assert.
        let lhs_idx = result.as_ref().unwrap().0;
        let lhs_val = *result.as_ref().unwrap().1;
        let rhs_idx = *indexs.iter().nth(index_pos).unwrap();
        let rhs_val = vec[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }

    fn with_slice() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = builder.build();
        let slice = context.fetch();
        let target = &mut slice.sparse_reader();

        // Act.
        let result = target.next();

        // Assert.
        let indexs = builder.npad_indexs();
        let lhs_idx = result.as_ref().unwrap().0;
        let lhs_val = *result.as_ref().unwrap().1;
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
        let target = SparseReader::<i32>::default();
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
            let vec = builder.build();
            let slice = vec.slice(range);
            let target = slice.sparse_reader();

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
        let target = &mut SparseReader::<i32>::default();
        let result = target.next_back();
        assert_eq!(result, None);
    }

    fn with_empty() {
        let vec = SparseVecSample::default();
        let target = &mut vec.sparse_reader();
        let result = target.next_back();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = SparseVecSample::normal();
        let target = &mut vec.sparse_reader();
        target.nth_back(vec.nnp() - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = builder.build();
        let indexs = builder.npad_indexs();
        let index_pos = indexs.len() / 2;
        let back_len = indexs.len() - index_pos - 1;
        let target = &mut vec.sparse_reader();
        target.nth_back(back_len - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        let lhs_idx = result.as_ref().unwrap().0;
        let lhs_val = *result.as_ref().unwrap().1;
        let rhs_idx = *indexs.iter().nth(index_pos).unwrap();
        let rhs_val = vec[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }

    fn with_slice() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = builder.build();
        let slice = context.fetch();
        let target = &mut slice.sparse_reader();

        // Act.
        let result = target.next_back();

        // Assert.
        let indexs = builder.npad_indexs();
        let lhs_idx = result.as_ref().unwrap().0;
        let lhs_val = *result.as_ref().unwrap().1;
        let rhs_idx = *indexs.last().unwrap();
        let rhs_val = builder.slice_values()[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }
}
