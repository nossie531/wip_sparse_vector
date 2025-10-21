use crate::tools::builder::*;
use crate::tools::helper;
use crate::tools::range;
use crate::tools::sample;
use std::ops::RangeFull;
use test_panic::prelude::*;

#[test]
fn is_empty() {
    with_zero_len();
    with_some_len();

    fn with_zero_len() {
        let vec = sample::default();
        let target = vec.slice(0..0);
        let result = target.is_empty();
        assert_eq!(result, true);
    }

    fn with_some_len() {
        let vec = sample::normal();
        let range = range::normal(vec.len());
        let target = vec.slice(range);
        let result = target.is_empty();
        assert_eq!(result, false);
    }
}

#[test]
fn len() {
    let vec = sample::normal();
    let range = range::normal(vec.len());
    let target = vec.slice(range.clone());
    let result = target.len();
    assert_eq!(result, range.len());
}

#[test]
fn iter() {
    let builder = SparseSliceBuilder::new();
    let context = builder.setup();
    let target = context.build();
    let result = target.iter();
    assert!(result.eq(builder.inside_values().iter()));
}

#[test]
fn sparse_reader() {
    // Arrange.
    let builder = SparseSliceBuilder::new();
    let context = builder.setup();
    let target = context.build();

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
    let context = builder.setup();
    let target = context.build();
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
        let vec = sample::normal();
        let outside = range::normal(vec.len());
        let inside = range::rev_order(outside.len());
        let target = vec.slice(outside.clone());
        let result = test_panic(|| target.slice(inside));
        assert!(result.is_panic());
    }

    fn with_range_end_gt_len() {
        let vec = sample::normal();
        let outside = range::normal(vec.len());
        let inside = range::gt_len(outside.len());
        let target = vec.slice(outside.clone());
        let result = test_panic(|| target.slice(inside));
        assert!(result.is_panic());
    }

    fn with_empty() {
        let vec = sample::normal();
        let outside = range::normal(vec.len());
        let inside = range::empty(outside.len());
        let target = vec.slice(outside.clone());
        let result = target.slice(inside.clone());
        assert!(result.is_empty());
    }

    fn with_all() {
        let builder = SparseSliceBuilder::new();
        let context = builder.setup();
        let target = context.build();
        let result = target.slice(RangeFull);
        assert_eq!(result.to_vec(), builder.inside_values());
    }

    fn with_normal() {
        let builder = SparseSliceBuilder::new();
        let range = range::normal(builder.inside_values().len());
        let context = builder.setup();
        let target = context.build();
        let result = target.slice(range.clone());
        assert_eq!(result.to_vec(), builder.inside_values()[range]);
    }
}

#[test]
fn hash() {
    for [vx, vy] in sample::pairs() {
        let sx = vx.slice(range::normal(vx.len()));
        let sy = vy.slice(range::normal(vy.len()));
        let result_x = helper::hash(&sx);
        let result_y = helper::hash(&sy);
        assert!(!sx.eq(&sy) || result_x == result_y);
    }
}
