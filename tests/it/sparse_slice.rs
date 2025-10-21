use crate::for_test::builders::*;
use crate::for_test::helper;
use crate::for_test::range;
use crate::for_test::samples::*;
use std::ops::RangeFull;
use test_panic::prelude::*;

#[test]
fn is_empty() {
    with_zero_len();
    with_some_len();

    fn with_zero_len() {
        let context = sample_ss::empty();
        let target = context.fetch();
        let result = target.is_empty();
        assert_eq!(result, true);
    }

    fn with_some_len() {
        let context = sample_ss::normal();
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
    assert_eq!(result, builder.inside_values().len());
}

#[test]
fn iter() {
    let builder = SparseSliceBuilder::new();
    let context = builder.build();
    let target = context.fetch();
    let result = target.iter();
    assert!(result.eq(builder.inside_values().iter()));
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
    let elms = builder.inside_values().into_iter().enumerate();
    let rhs = elms.filter(|e| e.1 != builder.padding());
    assert!(lhs.eq(rhs));
}

#[test]
fn to_vec() {
    let builder = SparseSliceBuilder::new();
    let context = builder.build();
    let target = context.fetch();
    let result = target.to_vec();
    assert_eq!(result, builder.inside_values());
}

#[test]
fn slice() {
    with_range_order_rev();
    with_range_end_gt_len();
    with_empty();
    with_all();
    with_normal();

    fn with_range_order_rev() {
        let context = sample_ss::normal();
        let target = context.fetch();
        let range = range::rev_order(target.len());
        let result = test_panic(|| target.slice(range));
        assert!(result.is_panic());
    }

    fn with_range_end_gt_len() {
        let context = sample_ss::normal();
        let target = context.fetch();
        let range = range::gt_len(target.len());
        let result = test_panic(|| target.slice(range));
        assert!(result.is_panic());
    }

    fn with_empty() {
        let context = sample_ss::normal();
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
        assert_eq!(result.to_vec(), builder.inside_values());
    }

    fn with_normal() {
        let builder = SparseSliceBuilder::new();
        let context = builder.build();
        let target = context.fetch();
        let range = range::normal(target.len());
        let result = target.slice(range.clone());
        assert_eq!(result.to_vec(), builder.inside_values()[range]);
    }
}

#[test]
fn hash() {
    for [vx, vy] in sample_sv::pairs() {
        let sx = vx.slice(range::normal(vx.len()));
        let sy = vy.slice(range::normal(vy.len()));
        let result_x = helper::hash(&sx);
        let result_y = helper::hash(&sy);
        assert!(!sx.eq(&sy) || result_x == result_y);
    }
}
