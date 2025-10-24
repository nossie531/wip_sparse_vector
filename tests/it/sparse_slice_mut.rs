use crate::for_test::helper;
use crate::for_test::range;
use crate::for_test::samples::*;
use crate::for_test::builders::*;
use test_panic::prelude::*;
use std::ops::Index;

#[test]
fn is_empty() {
    with_zero_len();
    with_some_len();

    fn with_zero_len() {
        let context = &mut SparseSliceSample::empty();
        let target = context.fetch_mut();
        let result = target.is_empty();
        assert_eq!(result, true);
    }

    fn with_some_len() {
        let context = &mut SparseSliceSample::normal();
        let target = context.fetch_mut();
        let result = target.is_empty();
        assert_eq!(result, false);
    }
}

#[test]
fn len() {
    let builder = SparseSliceBuilder::new();
    let context = &mut builder.build();
    let target = context.fetch_mut();
    let result = target.len();
    assert_eq!(result, builder.slice_values().len());
}

#[test]
fn to_vec() {
    let builder = SparseSliceBuilder::new();
    let context = &mut builder.build();
    let target = context.fetch_mut();
    let result = target.to_vec();
    assert_eq!(result, builder.slice_values());
}

#[test]
fn slice_ref() {
    let builder = SparseSliceBuilder::new();
    let context = &mut builder.build();
    let target = context.fetch_mut();
    let result = target.slice_ref();
    assert_eq!(result, &builder.build().fetch());
}

#[test]
fn slice() {
    with_range_order_rev();
    with_range_end_gt_len();
    with_empty();
    with_all();
    with_normal();

    fn with_range_order_rev() {
        let context = &mut SparseSliceSample::normal();
        let target = context.fetch_mut();
        let range = range::rev_order(target.len());
        let result = test_panic(|| target.slice(range));
        assert!(result.is_panic());
    }

    fn with_range_end_gt_len() {
        let context = &mut SparseSliceSample::normal();
        let target = context.fetch_mut();
        let range = range::gt_len(target.len());
        let result = test_panic(|| target.slice(range));
        assert!(result.is_panic());
    }

    fn with_empty() {
        let context = &mut SparseSliceSample::normal();
        let target = context.fetch_mut();
        let range = range::empty(target.len());
        let result = target.slice(range);
        assert!(result.is_empty());
    }

    fn with_all() {
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = context.fetch_mut();
        let result = target.slice(..);
        assert_eq!(result.to_vec(), builder.slice_values());
    }

    fn with_normal() {
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = context.fetch_mut();
        let range = range::normal(target.len());
        let result = target.slice(range.clone());
        assert_eq!(result.to_vec(), builder.slice_values()[range]);
    }
}

#[test]
fn iter() {
    let builder = SparseSliceBuilder::new();
    let context = &mut builder.build();
    let target = context.fetch_mut();
    let result = target.iter();
    assert!(result.eq(builder.slice_values().iter()));
}

#[test]
fn sparse_reader() {
    // Arrange.
    let builder = SparseSliceBuilder::new();
    let context = &mut builder.build();
    let target = context.fetch_mut();

    // Act.
    let result = target.sparse_reader();

    // Assert.
    let lhs = result.map(|e| (e.index(), *e.value()));
    let elms = builder.slice_values().into_iter().enumerate();
    let rhs = elms.filter(|e| e.1 != builder.padding());
    assert!(lhs.eq(rhs));
}

#[test]
fn sparse_writer() {
    // Arrange.
    let builder = SparseSliceBuilder::new();
    let context = &mut builder.build();
    let target = &mut context.fetch_mut();

    // Act.
    let result = target.sparse_writer();

    // Assert.
    let lhs = result.map(|e| (e.index(), *e.value()));
    let elms = builder.slice_values().into_iter().enumerate();
    let rhs = elms.filter(|e| e.1 != builder.padding());
    assert!(lhs.eq(rhs));
}

#[test]
fn take() {
    with_out_of_range();
    with_normal();
    with_padding();

    fn with_out_of_range() {
        let context = &mut SparseSliceSample::normal();
        let target = &mut context.fetch_mut();
        let result = test_panic(|| target.take(target.len()));
        assert!(result.is_panic());
    }

    fn with_normal() {
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let index = builder.some_npad_indexs(1)[0];
        let result = target.take(index);
        assert_eq!(result, builder.slice_values()[index]);
        assert_eq!(target[index], builder.padding());
    }

    fn with_padding() {
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let index = builder.some_pad_indexs(1)[0];
        let result = target.take(index);
        assert_eq!(result, builder.slice_values()[index]);
        assert_eq!(target[index], builder.padding());
    }
}

#[test]
fn edit() {
    with_out_of_range();
    with_normal();
    with_padding();

    fn with_out_of_range() {
        let context = &mut SparseSliceSample::normal();
        let target = &mut context.fetch_mut();
        let result = test_panic(|| target.edit(target.len()));
        assert!(result.is_panic());
    }

    fn with_normal() {
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let index = builder.some_npad_indexs(1)[0];
        let result = target.edit(index);
        assert_eq!(*result, builder.slice_values()[index]);
    }

    fn with_padding() {
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let index = builder.some_pad_indexs(1)[0];
        let result = target.edit(index);
        assert_eq!(*result, builder.slice_values()[index]);
    }
}

#[test]
fn fill() {
    with_normal();
    with_padding();

    fn with_normal() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let value = builder.none_padding();

        // Act.
        target.fill(value);

        // Assert.
        let lhs = context.vec().to_vec();
        let rhs = &mut builder.vec_values();
        rhs[builder.range()].fill(value);
        assert_eq!(&lhs, rhs);
    }

    fn with_padding() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let value = builder.padding();

        // Act.
        target.fill(value);

        // Assert.
        let lhs = context.vec().to_vec();
        let rhs = &mut builder.vec_values();
        rhs[builder.range()].fill(value);
        assert_eq!(&lhs, rhs);
    }
}

#[test]
fn fill_with() {
    // Arrange.
    let builder = SparseSliceBuilder::new();
    let context = &mut builder.build();
    let target = &mut context.fetch_mut();

    // Act.
    target.fill_with(|| 42);

    // Assert.
    let lhs = context.vec().to_vec();
    let rhs = &mut builder.vec_values();
    rhs[builder.range()].fill(42);
    assert_eq!(&lhs, rhs);
}

#[test]
fn swap() {
    with_arg1_out_of_range();
    with_arg2_out_of_range();
    with_arg1_eq_arg2();
    with_padding_and_padding();
    with_padding_and_value();
    with_value_and_value();

    fn with_arg1_out_of_range() {
        // Arrange.
        let context = &mut SparseSliceSample::normal();
        let target = &mut context.fetch_mut();
        let idx_x = target.len();
        let idx_y = target.len() / 2;

        // Act.
        let result = test_panic(|| target.swap(idx_x, idx_y));

        // Assert.
        assert!(result.is_panic());
    }

    fn with_arg2_out_of_range() {
        // Arrange.
        let context = &mut SparseSliceSample::normal();
        let target = &mut context.fetch_mut();
        let idx_x = target.len() / 2;
        let idx_y = target.len();

        // Act.
        let result = test_panic(|| target.swap(idx_x, idx_y));

        // Assert.
        assert!(result.is_panic());
    }

    fn with_arg1_eq_arg2() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let idx = target.len() / 2;

        // Act.
        target.swap(idx, idx);

        // Assert.
        assert_eq!(target.to_vec(), builder.slice_values());
    }

    fn with_padding_and_padding() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let idx_x = builder.some_pad_indexs(2)[0];
        let idx_y = builder.some_pad_indexs(2)[1];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut builder.slice_values();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }

    fn with_padding_and_value() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let idx_x = builder.some_pad_indexs(1)[0];
        let idx_y = builder.some_npad_indexs(1)[0];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut builder.slice_values();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }

    fn with_value_and_value() {
        // Arrange.
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = &mut context.fetch_mut();
        let idx_x = builder.some_npad_indexs(2)[0];
        let idx_y = builder.some_npad_indexs(2)[1];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut builder.slice_values();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }
}

#[test]
fn hash() {
    for [mut x, mut y] in SparseSliceSample::pairs() {
        let target_x = x.fetch_mut();
        let target_y = y.fetch_mut();
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
        let context = &mut SparseSliceSample::normal();
        let target = context.fetch_mut();
        let result = test_panic(|| target.index(target.len()));
        assert!(result.is_panic());
    }

    fn with_normal() {
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = context.fetch_mut();
        let index = builder.some_npad_indexs(1)[0];
        let result = target.index(index);
        assert_eq!(result, &builder.slice_values()[index]);
    }

    fn with_padding() {
        let builder = SparseSliceBuilder::new();
        let context = &mut builder.build();
        let target = context.fetch_mut();
        let index = builder.some_pad_indexs(1)[0];
        let result = target.index(index);
        assert_eq!(result, &builder.slice_values()[index]);
    }
}

#[test]
fn into_iter() {
    let builder = SparseSliceBuilder::new();
    let context = &mut builder.build();
    let target = &mut context.fetch_mut();
    let result = target.into_iter();
    assert!(result.eq(builder.slice_values().iter()));
}

#[test]
fn cmp() {
    for [mut xc, mut yc] in SparseSliceSample::pairs() {
        // Arrange.
        let [x, y] = [xc.fetch_mut(), yc.fetch_mut()];

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
        for [mut xc, mut yc] in SparseSliceSample::pairs() {
            // Arrange.
            let [x, y] = [xc.fetch_mut(), yc.fetch_mut()];

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
        let x = &mut SparseSliceSample::normal_floats();
        let y = &mut SparseSliceSample::normal_floats();
        let x = &x.fetch_mut();
        let y = &mut y.fetch_mut();
        *y.edit(x.len() / 2) = f32::NAN;

        // Act.
        let result_xy = PartialEq::eq(x, y);
        let result_yx = PartialEq::eq(y, x);

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
        for [mut xc, mut yc] in SparseSliceSample::pairs() {
            // Arrange.
            let [x, y] = [xc.fetch_mut(), yc.fetch_mut()];

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
        let x = &mut SparseSliceSample::normal_floats();
        let y = &mut SparseSliceSample::normal_floats();
        let x = &x.fetch_mut();
        let y = &mut y.fetch_mut();
        let index = y.len() / 2;
        *y.edit(index) = f32::NAN;

        // Act.
        let result_xy = PartialOrd::partial_cmp(x, y);
        let result_yx = PartialOrd::partial_cmp(y, x);

        // Assert.
        assert_eq!(result_xy, None);
        assert_eq!(result_yx, None);
    }
}
