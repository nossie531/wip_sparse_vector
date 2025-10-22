use crate::for_test::builders::*;
use crate::for_test::helper;
use crate::for_test::range;
use crate::for_test::samples::*;
use sparse_vector::prelude::*;
use upget::Upget;
use std::ops::{Bound, Index};
use test_panic::prelude::*;

#[test]
fn new() {
    let builder = SparseVecBuilder::default();
    let result = SparseVec::<i32>::new(builder.len());
    assert_eq!(result.len(), builder.len());
    assert_eq!(result.nnp(), 0);
    assert_eq!(result.padding(), &i32::default());
}

#[test]
fn with_padding() {
    let builder = SparseVecBuilder::default();
    let len = builder.len();
    let padding = builder.padding();
    let result = SparseVec::with_padding(len, padding);
    assert_eq!(result.len(), len);
    assert_eq!(result.nnp(), 0);
    assert_eq!(result.padding(), &padding);
}

#[test]
fn is_empty() {
    with_zero_len();
    with_some_len();

    fn with_zero_len() {
        let target = sample_sv::default();
        let result = target.is_empty();
        assert_eq!(result, true);
    }

    fn with_some_len() {
        let target = sample_sv::normal();
        let result = target.is_empty();
        assert_eq!(result, false);
    }
}

#[test]
fn is_all_padding() {
    with_all_padding();
    with_some_values();

    fn with_all_padding() {
        let target = sample_sv::all_padding();
        let result = target.is_all_padding();
        assert_eq!(result, true);
    }

    fn with_some_values() {
        let target = sample_sv::normal();
        let result = target.is_all_padding();
        assert_eq!(result, false);
    }
}

#[test]
fn len() {
    let builder = SparseVecBuilder::new();
    let target = builder.build();
    let result = target.len();
    assert_eq!(result, builder.len());
}

#[test]
fn nnp() {
    let builder = SparseVecBuilder::new();
    let target = builder.build();
    let result = target.nnp();
    assert_eq!(result, builder.nnp());
}

#[test]
fn padding() {
    with_default();
    with_normal();

    fn with_default() {
        let target = SparseVec::<i32>::default();
        let result = target.padding();
        assert_eq!(result, &i32::default());
    }

    fn with_normal() {
        let builder = SparseVecBuilder::new().set_padding(3);
        let target = builder.build();
        let result = target.padding();
        assert_eq!(result, &builder.padding());
    }
}

#[test]
fn clone_padding() {
    with_default();
    with_normal();

    fn with_default() {
        let target = SparseVec::<i32>::default();
        let result = target.clone_padding();
        assert_eq!(result, i32::default());
    }

    fn with_normal() {
        let builder = SparseVecBuilder::new().set_padding(3);
        let target = builder.build();
        let result = target.clone_padding();
        assert_eq!(result, builder.padding());
    }
}

#[test]
fn iter() {
    let builder = SparseVecBuilder::new();
    let target = builder.build();
    let result = target.iter();
    assert!(result.eq(builder.values().iter()));
}

#[test]
fn sparse_reader() {
    // Arrange.
    let builder = SparseVecBuilder::new();
    let target = builder.build();

    // Act.
    let result = target.sparse_reader();

    // Assert.
    let lft = result.map(|e| (e.index(), *e.value()));
    let rgt = builder.elms();
    assert!(lft.eq(rgt));
}

#[test]
fn to_vec() {
    let builder = SparseVecBuilder::new();
    let target = builder.build();
    let result = target.to_vec();
    assert_eq!(result, builder.values());
}

#[test]
fn slice() {
    with_range_order_rev();
    with_range_end_gt_len();
    with_empty();
    with_all();
    with_normal();

    fn with_range_order_rev() {
        let target = sample_sv::normal();
        let range = range::rev_order(target.len());
        let result = test_panic(|| target.slice(range));
        assert!(result.is_panic());
    }

    fn with_range_end_gt_len() {
        let target = sample_sv::normal();
        let range = range::gt_len(target.len());
        let result = test_panic(|| target.slice(range));
        assert!(result.is_panic());
    }

    fn with_empty() {
        let target = sample_sv::normal();
        let range = range::empty(target.len());
        let result = target.slice(range);
        assert_eq!(result.len(), 0);
    }

    fn with_all() {
        let target = sample_sv::normal();
        let result = target.slice(..);
        assert_eq!(result.len(), target.len());
    }

    fn with_normal() {
        let target = sample_sv::normal();
        let range = range::normal(target.len());
        let result = target.slice(range.clone());
        assert_eq!(result.len(), range.len());
    }
}

#[test]
fn set_len() {
    with_same();
    with_longer();
    with_shorter();
    with_zero();

    fn with_same() {
        // Arrange.
        let target = &mut sample_sv::normal();
        let value = target.len();

        // Act.
        target.set_len(value);

        // Assert.
        assert_eq!(target.len(), value);
        assert_eq!(target.iter().count(), value);
    }

    fn with_longer() {
        // Arrange.
        let target = &mut sample_sv::normal();
        let value = target.len() + 1;
        let padding = *target.padding();
        let original = target.iter().cloned().collect::<Vec<_>>();

        // Act.
        target.set_len(value);

        // Assert.
        let expecteds = [original, vec![padding]].concat();
        assert!(target.iter().eq(expecteds.iter()));
    }

    fn with_shorter() {
        // Arrange.
        let target = &mut sample_sv::normal();
        let value = target.len() - 1;

        // Act.
        target.set_len(value);

        // Assert.
        assert_eq!(target.len(), value);
        assert_eq!(target.iter().count(), value);
    }

    fn with_zero() {
        // Arrange.
        let target = &mut sample_sv::normal();

        // Act.
        target.set_len(0);

        // Assert.
        assert_eq!(target.len(), 0);
        assert_eq!(target.iter().count(), 0);
    }
}

#[test]
fn slice_mut() {
    with_range_order_rev();
    with_range_end_gt_len();
    with_empty();
    with_all();
    with_normal();

    fn with_range_order_rev() {
        let target = &mut sample_sv::normal();
        let start = Bound::Excluded(target.len() / 2);
        let end = Bound::Excluded(target.len() / 2);
        let result = test_panic(|| target.slice_mut((start, end)));
        assert!(result.is_panic());
    }

    fn with_range_end_gt_len() {
        let target = &mut sample_sv::normal();
        let start = target.len() / 2;
        let end = target.len() + 1;
        let result = test_panic(|| target.slice_mut(start..end));
        assert!(result.is_panic());
    }

    fn with_empty() {
        let target = &mut sample_sv::normal();
        let index = target.len() / 2;
        let result = target.slice_mut(index..index);
        assert_eq!(result.len(), 0);
    }

    fn with_all() {
        let target = &mut sample_sv::normal();
        let result = target.slice_mut(..);
        assert_eq!(result.len(), target.len());
    }

    fn with_normal() {
        let target = &mut sample_sv::normal();
        let start = target.len() / 3 * 1;
        let end = target.len() / 3 * 2;
        let range = start..end;
        let result = target.slice_mut(range.clone());
        assert_eq!(result.len(), range.len());
    }
}

#[test]
fn sparse_writer() {
    // Arrange.
    let builder = SparseVecBuilder::new();
    let target = &mut builder.build();

    // Act.
    let result = &mut target.sparse_writer();

    // Assert.
    let lhs = helper::vec_from_sparse_writer(result);
    let rhs = builder.elms();
    assert_eq!(lhs, rhs);
}

#[test]
fn take() {
    with_out_of_range();
    with_normal();
    with_padding();

    fn with_out_of_range() {
        // Arrange.
        let target = &mut sample_sv::normal();
        let index = target.len();

        // Act.
        let result = test_panic(|| target.take(index));

        // Assert.
        assert!(result.is_panic());
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let index = SparseVecBuilder::new().some_npad_indexs(1)[0];

        // Act.
        let result = target.take(index);

        // Assert.
        assert_eq!(result, builder.values()[index]);
        assert_eq!(target[index], builder.padding());
        assert_eq!(target.len(), builder.len());
        assert_eq!(target.nnp(), builder.nnp() - 1);
    }

    fn with_padding() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let index = SparseVecBuilder::new().some_pad_indexs(1)[0];

        // Act.
        let result = target.take(index);

        // Assert.
        assert_eq!(result, builder.padding());
        assert_eq!(target[index], builder.padding());
        assert_eq!(target.len(), builder.len());
        assert_eq!(target.nnp(), builder.nnp());
    }
}

#[test]
fn edit() {
    with_out_of_range();
    with_normal();
    with_padding();

    fn with_out_of_range() {
        // Arrange.
        let target = &mut sample_sv::normal();
        let index = target.len();

        // Act.
        let result = test_panic(|| target.edit(index));

        // Assert.
        assert!(result.is_panic());
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let index = SparseVecBuilder::new().some_npad_indexs(1)[0];

        // Act.
        let result = target.edit(index);

        // Assert.
        assert_eq!(*result, builder.values()[index]);
    }

    fn with_padding() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let index = SparseVecBuilder::new().some_pad_indexs(1)[0];

        // Act.
        let result = target.edit(index);

        // Assert.
        assert_eq!(*result, builder.values()[index]);
    }
}

#[test]
fn pop() {
    with_empty();
    with_last_normal();
    with_last_padding();

    fn with_empty() {
        // Arrange.
        let target = &mut sample_sv::default();

        // Act.
        let result = target.pop();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_last_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new().set_padding(0);
        let value = builder.none_padding();
        let values = [builder.values(), vec![value]].concat();
        let target = &mut SparseVec::from_iter(values);
        let len = target.len();

        // Act.
        let result = target.pop();

        // Assert.
        assert_eq!(result, Some(value));
        assert_eq!(target.len(), len - 1);
    }

    fn with_last_padding() {
        // Arrange.
        let builder = SparseVecBuilder::new().set_padding(0);
        let padding = builder.padding();
        let values = [builder.values(), vec![padding]].concat();
        let target = &mut SparseVec::from_iter(values);
        let len = target.len();

        // Act.
        let result = target.pop();

        // Assert.
        assert_eq!(result, Some(builder.padding()));
        assert_eq!(target.len(), len - 1)
    }
}

#[test]
fn push() {
    with_normal();
    with_padding();

    fn with_normal() {
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let value = builder.none_padding();
        target.push(value);
        assert_eq!(target.len(), builder.len() + 1);
        assert_eq!(target[target.len() - 1], value);
    }

    fn with_padding() {
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let padding = builder.padding();
        target.push(padding);
        assert_eq!(target.len(), builder.len() + 1);
        assert_eq!(target[target.len() - 1], padding);
    }
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
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let idx_x = builder.len();
        let idx_y = builder.len() / 2;

        // Act.
        let result = test_panic(|| {
            target.swap(idx_x, idx_y);
        });

        // Assert.
        assert!(result.is_panic());
    }

    fn with_arg2_out_of_range() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let idx_x = builder.len() / 2;
        let idx_y = builder.len();

        // Act.
        let result = test_panic(|| {
            target.swap(idx_x, idx_y);
        });

        // Assert.
        assert!(result.is_panic());
    }

    fn with_arg1_eq_arg2() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let idx = builder.len() / 2;

        // Act.
        target.swap(idx, idx);

        // Assert.
        assert_eq!(target.to_vec(), builder.values());
    }

    fn with_padding_and_padding() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let idx_x = builder.some_pad_indexs(2)[0];
        let idx_y = builder.some_pad_indexs(2)[1];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut builder.values();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }

    fn with_padding_and_value() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let idx_x = builder.some_pad_indexs(1)[0];
        let idx_y = builder.some_npad_indexs(1)[0];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut builder.values();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }

    fn with_value_and_value() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let idx_x = builder.some_npad_indexs(2)[0];
        let idx_y = builder.some_npad_indexs(2)[1];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut builder.values();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }
}

#[test]
fn fill() {
    // Arrange.
    let builder = SparseVecBuilder::new();
    let target = &mut builder.build();

    // Act.
    target.fill(42);

    // Assert.
    let rhs = &mut builder.values();
    rhs.fill(42);
    assert_eq!(target.to_vec(), *rhs);
}

#[test]
fn fill_with() {
    // Arrange.
    let builder = SparseVecBuilder::new();
    let target = &mut builder.build();

    // Act.
    target.fill_with(|| 42);

    // Assert.
    let rhs = &mut builder.values();
    rhs.fill(42);
    assert_eq!(target.to_vec(), *rhs);
}

#[test]
fn splice() {
    // Arrange.
    let builder = SparseVecBuilder::new();
    let target = &mut builder.build();
    let range = range::normal(target.len());
    let inserts = sample_vec::normal(range.len() / 2);

    // Act.
    let result = target.splice(range.clone(), inserts.clone());

    // Assert result.
    let values = builder.values();
    let rhs = values[range.clone()].iter().copied();
    assert!(result.eq(rhs));

    // Assert target changes.
    let rhs = values.clone().upget(|x| { x.splice(range.clone(), inserts); });
    assert_eq!(target.to_vec(), rhs);
}

#[test]
fn default() {
    let result = SparseVec::<i32>::default();
    assert_eq!(result.len(), 0);
    assert_eq!(result.padding(), &0);
}

#[test]
fn extend() {
    with_value();
    with_ref();

    fn with_value() {
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let vec = vec![1, 2, 3];
        target.extend(vec.clone());

        let sample_vec = builder.values();
        let rhs = sample_vec.iter().chain(vec.iter());
        assert!(target.iter().eq(rhs));
    }

    fn with_ref() {
        let builder = SparseVecBuilder::new();
        let target = &mut builder.build();
        let vec = &vec![1, 2, 3];
        target.extend(vec);

        let sample_vec = builder.values();
        let rhs = sample_vec.iter().chain(vec);
        assert!(target.iter().eq(rhs));
    }
}

#[test]
fn from() {
    with_arr();
    with_vec();

    fn with_arr() {
        let arr = ValuesBuilder::new().array();
        let result = SparseVec::from(arr.clone());
        assert!(result.iter().eq(arr.iter()));
    }

    fn with_vec() {
        let vec = ValuesBuilder::new().values();
        let result = SparseVec::from(vec.clone());
        assert!(result.iter().eq(vec.iter()));
    }
}

#[test]
fn from_iter() {
    let builder = SparseVecBuilder::new();
    let vec = builder.build();
    let iter = vec.iter().cloned();
    let result = SparseVec::from_iter(iter);
    assert_eq!(result.len(), builder.len());
    assert!(result.iter().eq(vec.iter()));
}

#[test]
fn hash() {
    for pair in sample_sv::pairs() {
        let [x, y] = pair;
        let result_x = helper::hash(&x);
        let result_y = helper::hash(&y);
        assert!(!x.eq(&y) || result_x == result_y);
    }
}

#[test]
fn index() {
    with_out_of_range();
    with_normal();
    with_padding();

    fn with_out_of_range() {
        // Arrange.
        let target = sample_sv::normal();
        let index = target.len();

        // Act.
        let result = test_panic(|| target.index(index));

        // Assert.
        assert!(result.is_panic());
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = builder.build();
        let index = builder.some_npad_indexs(1)[0];

        // Act.
        let result = target.index(index);

        // Assert.
        assert_eq!(result, &builder.values()[index]);
    }

    fn with_padding() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let target = builder.build();
        let index = builder.some_pad_indexs(1)[0];

        // Act.
        let result = target.index(index);

        // Assert.
        assert_eq!(result, &builder.values()[index]);
    }
}

#[test]
fn into_iter() {
    with_value();
    with_ref();

    fn with_value() {
        let builder = SparseVecBuilder::new();
        let target = builder.build();
        let result = target.into_iter();
        assert!(result.eq(builder.values()));
    }

    fn with_ref() {
        let builder = SparseVecBuilder::new();
        let target = &SparseVecBuilder::new().build();
        let result = target.into_iter();
        assert!(result.eq(builder.values().iter()));
    }
}

#[test]
fn cmp() {
    for pair in sample_sv::pairs() {
        // Arrange.
        let [x, y] = pair;

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
        for pair in sample_sv::pairs() {
            // Arrange.
            let [x, y] = pair;

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
        let x = &sample_sv::normal_floats();
        let y = &mut sample_sv::normal_floats();
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
        for pair in sample_sv::pairs() {
            // Arrange.
            let [x, y] = pair;

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
        let x = &sample_sv::normal_floats();
        let y = &mut sample_sv::normal_floats();
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

#[test]
fn from_for_vec() {
    let sparse_vec = sample_sv::normal();
    let result = <Vec<_> as From<SparseVec<_>>>::from(sparse_vec.clone());
    assert!(result.iter().eq(sparse_vec.iter()));
}
