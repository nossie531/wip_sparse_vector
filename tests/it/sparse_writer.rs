use std::mem;

use crate::for_test::sample as ts;
use crate::for_test::template as tt;
use sparse_vector::SparseWriter;

#[test]
fn default() {
    let result = SparseWriter::<i32>::default();
    assert_eq!(result.size_hint(), (0, Some(0)));
    assert_eq!(result.count(), 0);
}

#[test]
fn drop() {
    // Arrange vec.
    let template = tt::template();
    let vec = &mut template.build();

    // Arrange writer.
    let mut target = vec.sparse_writer();
    let index = template.sample_values_set().iter().count() / 2;
    let elm = &mut target.nth(index).unwrap();
    *elm.value_mut() = template.padding();

    // Act.
    mem::drop(target);

    // Assert.
    assert_eq!(vec.nnp(), template.nnp() - 1);
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
        let vec = &mut ts::default();
        let target = &mut vec.sparse_writer();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let template = tt::template();
        let vec = &mut template.build();
        let target = &mut vec.sparse_writer();
        target.nth(template.nnp() - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let template = tt::template();
        let vec = &mut template.build();
        let indexs = template.sample_values_set();
        let index_pos = indexs.len() / 2;
        let target = &mut vec.sparse_writer();
        target.nth(index_pos - 1);

        // Act.
        let result = target.next();

        // Assert.
        let lhs_idx = result.as_ref().unwrap().index();
        let lhs_val = *result.as_ref().unwrap().value();
        let rhs_idx = *indexs.iter().nth(index_pos).unwrap();
        let rhs_val = template.sample_vec()[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }

    fn with_slice() {
        // Arrange.
        let template = tt::template();
        let range = ts::range(template.len());
        let vec = &mut template.build();
        let slice = &mut vec.slice_mut(range.clone());
        let target = &mut slice.sparse_writer();

        // Act.
        let result = target.next();

        // Assert.
        let indexs = template.sample_values_set();
        let indexs = &mut indexs.iter().copied();
        let lhs_idx = result.as_ref().unwrap().index();
        let lhs_val = *result.as_ref().unwrap().value();
        let rhs_idx = indexs.find(|x| *x >= range.start).unwrap() - range.start;
        let rhs_val = template.sample_vec()[range.start + rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
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
        let vec = &mut ts::default();
        let target = &mut vec.sparse_writer();
        let result = target.next_back();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let template = tt::template();
        let vec = &mut template.build();
        let target = &mut vec.sparse_writer();
        target.nth_back(template.nnp() - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let template = tt::template();
        let vec = &mut template.build();
        let indexs = template.sample_values_set();
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
        let rhs_val = template.sample_vec()[rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }

    fn with_slice() {
        // Arrange.
        let template = tt::template();
        let range = ts::range(template.len());
        let vec = &mut template.build();
        let slice = &mut vec.slice_mut(range.clone());
        let target = &mut slice.sparse_writer();

        // Act.
        let result = target.next_back();

        // Assert.
        let indexs = template.sample_values_set();
        let indexs = &mut indexs.iter().copied();
        let lhs_idx = result.as_ref().unwrap().index();
        let lhs_val = *result.as_ref().unwrap().value();
        let rhs_idx = indexs.rfind(|x| *x < range.end).unwrap() - range.start;
        let rhs_val = template.sample_vec()[range.start + rhs_idx];
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }
}