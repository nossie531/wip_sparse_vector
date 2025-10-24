use crate::for_test::builders::*;
use crate::for_test::helper;
use crate::for_test::range;
use crate::for_test::samples::*;
use std::ops::{Index, RangeFull};
use test_panic::prelude::*;

#[test]
fn is_empty() {
    with_zero_len();
    with_some_len();

    fn with_zero_len() {
        let context = SparseSliceSample::empty();
        let target = context.fetch();
        let result = target.is_empty();
        assert_eq!(result, true);
    }

    fn with_some_len() {
        let context = SparseSliceSample::normal();
        let target = context.fetch();
        let result = target.is_empty();
        assert_eq!(result, false);
    }
}

#[test]
fn len() {
    let builder = SparseSliceBuilder::new();
    let context = builder.build();
    let target = context.fetch();
    let result = target.len();
    assert_eq!(result, builder.slice_values().len());
}

#[test]
fn iter() {
    let builder = SparseSliceBuilder::new();
    let context = builder.build();
    let target = context.fetch();
    let result = target.iter();
    assert!(result.eq(builder.slice_values().iter()));
}

#[test]
fn sparse_reader() {
    // Arrange.
    let builder = SparseSliceBuilder::new();
    let context = builder.build();
    let target = context.fetch();

    // Act.
    let result = target.sparse_reader();

    // Assert.
    let lhs = result.map(|e| (e.index(), *e.value()));
    let elms = builder.slice_values().into_iter().enumerate();
    let rhs = elms.filter(|e| e.1 != builder.padding());
    assert!(lhs.eq(rhs));
}

#[test]
fn to_vec() {
    let builder = SparseSliceBuilder::new();
    let context = builder.build();
    let target = context.fetch();
    let result = target.to_vec();
    assert_eq!(result, builder.slice_values());
}

#[test]
fn slice() {
    with_range_order_rev();
    with_range_end_gt_len();
    with_empty();
    with_all();
    with_normal();

    fn with_range_order_rev() {
        let context = SparseSliceSample::normal();
        let target = context.fetch();
        let range = range::rev_order(target.len());
        let result = test_panic(|| target.slice(range));
        assert!(result.is_panic());
    }

    fn with_range_end_gt_len() {
        let context = SparseSliceSample::normal();
        let target = context.fetch();
        let range = range::gt_len(target.len());
        let result = test_panic(|| target.slice(range));
        assert!(result.is_panic());
    }

    fn with_empty() {
        let context = SparseSliceSample::normal();
        let target = context.fetch();
        let range = range::empty(target.len());
        let result = target.slice(range);
        assert!(result.is_empty());
    }

    fn with_all() {
        let builder = SparseSliceBuilder::new();
        let context = builder.build();
        let target = context.fetch();
        let result = target.slice(RangeFull);
        assert_eq!(result.to_vec(), builder.slice_values());
    }

    fn with_normal() {
        let builder = SparseSliceBuilder::new();
        let context = builder.build();
        let target = context.fetch();
        let range = range::normal(target.len());
        let result = target.slice(range.clone());
        assert_eq!(result.to_vec(), builder.slice_values()[range]);
    }
}

#[test]
fn hash() {
    for [x, y] in SparseSliceSample::pairs() {
        let target_x = x.fetch();
        let target_y = y.fetch();
        let result_x = helper::hash(&target_x);
        let result_y = helper::hash(&target_y);
        assert!(!target_x.eq(&target_y) || result_x == result_y);
    }
}

#[test]
fn index() {
    with_out_of_range();
    with_normal();
    with_padding();

    fn with_out_of_range() {
        let context = SparseSliceSample::normal();
        let target = context.fetch();
        let result = test_panic(|| target.index(target.len()));
        assert!(result.is_panic());
    }

    fn with_normal() {
        let builder = SparseSliceBuilder::new();
        let context = builder.build();
        let target = context.fetch();
        let index = builder.some_npad_indexs(1)[0];
        let result = target.index(index);
        assert_eq!(result, &builder.slice_values()[index]);
    }

    fn with_padding() {
        let builder = SparseSliceBuilder::new();
        let context = builder.build();
        let target = context.fetch();
        let index = builder.some_pad_indexs(1)[0];
        let result = target.index(index);
        assert_eq!(result, &builder.slice_values()[index]);
    }
}

#[test]
fn into_iter() {
    let builder = SparseSliceBuilder::new();
    let context = builder.build();
    let target = context.fetch();
    let result = target.into_iter();
    assert!(result.eq(builder.slice_values().iter()));
}

#[test]
fn cmp() {
    for pair in SparseSliceSample::pairs() {
        // Arrange.
        let [x, y] = [pair[0].fetch(), pair[1].fetch()];

        // Act.
        let result_xy = Ord::cmp(&x, &y);
        let result_yx = Ord::cmp(&y, &x);

        // Assert.
        let expected_xy = Ord::cmp(&x.to_vec(), &y.to_vec());
        let expected_yx = Ord::cmp(&y.to_vec(), &x.to_vec());
        assert_eq!(result_xy, expected_xy);
        assert_eq!(result_yx, expected_yx);
    }
}

#[test]
fn eq() {
    with_normal();
    with_nan();

    fn with_normal() {
        for pair in SparseSliceSample::pairs() {
            // Arrange.
            let [x, y] = [pair[0].fetch(), pair[1].fetch()];

            // Act.
            let result_xy = PartialEq::eq(&x, &y);
            let result_yx = PartialEq::eq(&y, &x);

            // Assert.
            let expected_xy = PartialEq::eq(&x.to_vec(), &y.to_vec());
            let expected_yx = PartialEq::eq(&y.to_vec(), &x.to_vec());
            assert_eq!(result_xy, expected_xy);
            assert_eq!(result_yx, expected_yx);
        }
    }

    fn with_nan() {
        // Arrange.
        let x = &SparseSliceSample::normal_floats();
        let y = &mut SparseSliceSample::normal_floats();
        let x = &x.fetch();
        let y = &mut y.fetch_mut();
        *y.edit(x.len() / 2) = f32::NAN;

        // Act.
        let result_xy = PartialEq::eq(x, y.slice_ref());
        let result_yx = PartialEq::eq(y.slice_ref(), x);

        // Assert.
        assert_eq!(result_xy, false);
        assert_eq!(result_yx, false);
    }
}

#[test]
fn partial_cmp() {
    with_normal();
    with_nan();

    fn with_normal() {
        for pair in SparseSliceSample::pairs() {
            // Arrange.
            let [x, y] = [pair[0].fetch(), pair[1].fetch()];

            // Act.
            let result_xy = PartialOrd::partial_cmp(&x, &y);
            let result_yx = PartialOrd::partial_cmp(&y, &x);

            // Assert.
            let expected_xy = PartialOrd::partial_cmp(&x.to_vec(), &y.to_vec());
            let expected_yx = PartialOrd::partial_cmp(&y.to_vec(), &x.to_vec());
            assert_eq!(result_xy, expected_xy);
            assert_eq!(result_yx, expected_yx);
        }
    }

    fn with_nan() {
        // Arrange.
        let x = &SparseSliceSample::normal_floats();
        let y = &mut SparseSliceSample::normal_floats();
        let x = &x.fetch();
        let y = &mut y.fetch_mut();
        let index = y.len() / 2;
        *y.edit(index) = f32::NAN;

        // Act.
        let result_xy = PartialOrd::partial_cmp(x, y.slice_ref());
        let result_yx = PartialOrd::partial_cmp(y.slice_ref(), x);

        // Assert.
        assert_eq!(result_xy, None);
        assert_eq!(result_yx, None);
    }
}
