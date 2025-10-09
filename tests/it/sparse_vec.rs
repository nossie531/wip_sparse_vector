use core::f32;

use crate::for_test::helper as th;
use crate::for_test::sample as ts;
use crate::for_test::template as tt;
use sparse_vec::prelude::*;
use test_panic::prelude::*;

#[test]
fn new() {
    let result = SparseVec::<i32>::new(tt::LEN);
    assert_eq!(result.len(), tt::LEN);
    assert_eq!(result.nnp(), 0);
    assert_eq!(result.padding(), &i32::default());
}

#[test]
fn with_padding() {
    let result = SparseVec::with_padding(tt::LEN, tt::PADDING);
    assert_eq!(result.len(), tt::LEN);
    assert_eq!(result.nnp(), 0);
    assert_eq!(result.padding(), &tt::PADDING);
}

#[test]
fn is_empty() {
    with_zero_len();
    with_some_len();

    fn with_zero_len() {
        let target = ts::default();
        let result = target.is_empty();
        assert_eq!(result, true);
    }

    fn with_some_len() {
        let target = ts::normal();
        let result = target.is_empty();
        assert_eq!(result, false);
    }
}

#[test]
fn is_all_padding() {
    with_all_padding();
    with_some_values();

    fn with_all_padding() {
        let target = ts::all_padding();
        let result = target.is_all_padding();
        assert_eq!(result, true);
    }

    fn with_some_values() {
        let target = ts::normal();
        let result = target.is_all_padding();
        assert_eq!(result, false);
    }
}

#[test]
fn len() {
    let template = tt::template();
    let target = template.build();
    let result = target.len();
    assert_eq!(result, template.len());
}

#[test]
fn nnp() {
    let template = tt::template();
    let target = template.build();
    let result = target.nnp();
    assert_eq!(result, template.nnp());
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
        let template = tt::template().set_padding(3);
        let target = template.build();
        let result = target.padding();
        assert_eq!(result, &template.padding());
    }
}

#[test]
fn iter() {
    let template = tt::template();
    let target = template.build();
    let result = target.iter();
    assert!(result.eq(template.sample_vec().iter()));
    // ✏️ TODO: 全てが既定値のテストとかは…。イテレータ側かなこれは…。
}

#[test]
fn sparse_reader() {
    // Arrange.
    let template = tt::template();
    let target = template.build();

    // Act.
    let result = target.sparse_reader();

    // Assert.
    let lft = result.map(|e| (e.index(), *e.value()));
    let rgt = template.sample_elms();
    assert!(lft.eq(rgt));
}

#[test]
fn edit() {
    with_out_of_range();
    with_normal();
    with_padding();

    fn with_out_of_range() {
        // Arrange.
        let target = &mut ts::normal();
        let index = target.len();

        // Act.
        let result = test_panic(|| target.edit(index));

        // Assert.
        assert!(result.is_panic());
    }

    fn with_normal() {
        // Arrange.
        let template = tt::template();
        let target = &mut template.build();
        let index = tt::template().sample_value_indexs(1)[0];

        // Act.
        let result = target.edit(index);

        // Assert.
        assert_eq!(*result, template.sample_vec()[index]);
    }

    fn with_padding() {
        // Arrange.
        let template = tt::template();
        let target = &mut template.build();
        let index = tt::template().sample_padding_indexs(1)[0];

        // Act.
        let result = target.edit(index);

        // Assert.
        assert_eq!(*result, template.sample_vec()[index]);
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
        let target = &mut ts::normal();
        let value = target.len();

        // Act.
        target.set_len(value);

        // Assert.
        assert_eq!(target.len(), value);
        assert_eq!(target.iter().count(), value);
    }

    fn with_longer() {
        // Arrange.
        let target = &mut ts::normal();
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
        let target = &mut ts::normal();
        let value = target.len() - 1;

        // Act.
        target.set_len(value);

        // Assert.
        assert_eq!(target.len(), value);
        assert_eq!(target.iter().count(), value);
    }

    fn with_zero() {
        // Arrange.
        let target = &mut ts::normal();

        // Act.
        target.set_len(0);

        // Assert.
        assert_eq!(target.len(), 0);
        assert_eq!(target.iter().count(), 0);
    }
}

#[test]
fn sparse_writer() {
    // Arrange.
    let template = tt::template();
    let target = &mut template.build();

    // Act.
    let result = &mut target.sparse_writer();

    // Assert.
    let lhs = th::vec_from_sparse_writer(result);
    let rhs = template.sample_elms();
    assert_eq!(lhs, rhs);
}

#[test]
fn to_vec() {
    // Arrange.
    let template = tt::template();
    let target = template.build();

    // Act.
    let result = target.to_vec();

    // Assert.
    assert_eq!(result, template.sample_vec());
}

#[test]
fn fill() {
    // Arrange.
    let template = tt::template();
    let target = &mut template.build();

    // Act.
    target.fill(42);

    // Assert.
    let rhs = &mut template.sample_vec();
    rhs.fill(42);
    assert_eq!(target.to_vec(), *rhs);
}

#[test]
fn fill_with() {
    // Arrange.
    let template = tt::template();
    let target = &mut template.build();

    // Act.
    target.fill_with(|| 42);

    // Assert.
    let rhs = &mut template.sample_vec();
    rhs.fill(42);
    assert_eq!(target.to_vec(), *rhs);
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
        let template = tt::template();
        let target = &mut template.build();
        let idx_x = template.len();
        let idx_y = template.len() / 2;

        // Act.
        let result = test_panic(|| {
            target.swap(idx_x, idx_y);
        });

        // Assert.
        assert!(result.is_panic());
    }

    fn with_arg2_out_of_range() {
        // Arrange.
        let template = tt::template();
        let target = &mut template.build();
        let idx_x = template.len() / 2;
        let idx_y = template.len();

        // Act.
        let result = test_panic(|| {
            target.swap(idx_x, idx_y);
        });

        // Assert.
        assert!(result.is_panic());
    }

    fn with_arg1_eq_arg2() {
        // Arrange.
        let template = tt::template();
        let target = &mut template.build();
        let idx = template.len() / 2;

        // Act.
        target.swap(idx, idx);

        // Assert.
        assert_eq!(target.to_vec(), template.sample_vec());
    }

    fn with_padding_and_padding() {
        // Arrange.
        let template = tt::template();
        let target = &mut template.build();
        let idx_x = template.sample_padding_indexs(2)[0];
        let idx_y = template.sample_padding_indexs(2)[1];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut template.sample_vec();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }

    fn with_padding_and_value() {
        // Arrange.
        let template = tt::template();
        let target = &mut template.build();
        let idx_x = template.sample_padding_indexs(1)[0];
        let idx_y = template.sample_value_indexs(1)[0];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut template.sample_vec();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }

    fn with_value_and_value() {
        // Arrange.
        let template = tt::template();
        let target = &mut template.build();
        let idx_x = template.sample_value_indexs(2)[0];
        let idx_y = template.sample_value_indexs(2)[1];

        // Act.
        target.swap(idx_x, idx_y);

        // Assert.
        let rhs = &mut template.sample_vec();
        rhs.swap(idx_x, idx_y);
        assert_eq!(target.to_vec(), *rhs);
    }
}

#[test]
fn default() {
    let result = SparseVec::<i32>::default();
    assert_eq!(result.len(), 0);
    assert_eq!(result.padding(), &0);
}

#[test]
fn from() {
    with_arr();
    with_vec();

    fn with_arr() {
        let arr = tt::template().sample_arr();
        let result = SparseVec::from(arr.clone());
        assert!(result.iter().eq(arr.iter()));
    }

    fn with_vec() {
        let vec = tt::template().sample_vec();
        let result = SparseVec::from(vec.clone());
        assert!(result.iter().eq(vec.iter()));
    }
}

#[test]
fn from_iter() {
    let vec = tt::template().sample_vec();
    let iter = vec.iter().cloned();
    let result = SparseVec::from_iter(iter);
    assert!(result.iter().eq(vec.iter()));
}

#[test]
fn index() {
    with_out_of_range();
    with_normal();
    with_padding();

    fn with_out_of_range() {
        // Arrange.
        let target = ts::normal();
        let index = target.len();

        // Act.
        let result = test_panic(|| target[index]);

        // Assert.
        assert!(result.is_panic());
    }

    fn with_normal() {
        // Arrange.
        let template = tt::template();
        let target = template.build();
        let index = template.sample_value_indexs(1)[0];

        // Act.
        let result = target[index];

        // Assert.
        assert_eq!(result, tt::template().sample_vec()[index]);
    }

    fn with_padding() {
        // Arrange.
        let template = tt::template();
        let target = template.build();
        let index = template.sample_padding_indexs(1)[0];

        // Act.
        let result = target[index];

        // Assert.
        assert_eq!(result, tt::template().sample_vec()[index]);
    }
}

#[test]
fn into_iter() {
    with_value();
    with_ref();

    fn with_value() {
        let template = tt::template();
        let target = template.build();
        let result = target.into_iter();
        assert!(result.eq(template.sample_vec()));
    }

    fn with_ref() {
        let template = tt::template();
        let target = &tt::template().build();
        let result = target.into_iter();
        assert!(result.eq(template.sample_vec().iter()));
    }
}

#[test]
fn cmp() {
    for pair in ts::pairs() {
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
fn partial_eq() {
    with_normal();
    with_nan();

    fn with_normal() {
        for pair in ts::pairs() {
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
        let x = &ts::normal_floats();
        let y = &mut ts::normal_floats();
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
        for pair in ts::pairs() {
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
        let x = &ts::normal_floats();
        let y = &mut ts::normal_floats();
        *y.edit(y.len() / 2) = f32::NAN;

        // Act.
        let result_xy = PartialOrd::partial_cmp(x, y);
        let result_yx = PartialOrd::partial_cmp(y, x);

        // Assert.
        assert_eq!(result_xy, None);
        assert_eq!(result_yx, None);
    }
}

#[test]
fn extend() {
    with_value();
    with_ref();

    fn with_value() {
        let template = tt::template();
        let target = &mut template.build();
        let vec = vec![1, 2, 3];
        target.extend(vec.clone());

        let sample_vec = template.sample_vec();
        let rhs = sample_vec.iter().chain(vec.iter());
        assert!(target.iter().eq(rhs));
    }

    fn with_ref() {
        let template = tt::template();
        let target = &mut template.build();
        let vec = &vec![1, 2, 3];
        target.extend(vec);

        let sample_vec = template.sample_vec();
        let rhs = sample_vec.iter().chain(vec);
        assert!(target.iter().eq(rhs));
    }
}

#[test]
fn from_for_vec() {
    let sparse_vec = ts::normal();
    let result = <Vec<_> as From<SparseVec<_>>>::from(sparse_vec.clone());
    assert!(result.iter().eq(sparse_vec.iter()));
}
