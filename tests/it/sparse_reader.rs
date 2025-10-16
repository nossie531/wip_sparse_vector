use crate::for_test::sample as ts;
use crate::for_test::template as tt;
use sparse_vector::SparseReader;

#[test]
fn clone() {
    let vec = ts::normal();
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
    with_empty();
    with_overrun();
    with_normal();

    fn with_empty() {
        let vec = ts::default();
        let target = &mut vec.sparse_reader();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = ts::normal();
        let target = &mut vec.sparse_reader();
        target.nth(vec.nnp() - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let template = tt::template();
        let vec = template.build();
        let indexs = template.sample_values_set();
        let index_pos = indexs.len() / 2;
        let target = &mut vec.sparse_reader();
        target.nth(index_pos - 1);

        // Act.
        let result = target.next();

        // Assert.
        let rhs_idx = *indexs.iter().nth(index_pos).unwrap();
        let rhs_val = vec[rhs_idx];
        let lhs_idx = result.as_ref().unwrap().index();
        let lhs_val = *result.as_ref().unwrap().value();
        assert_eq!(lhs_idx, rhs_idx);
        assert_eq!(lhs_val, rhs_val);
    }
}

#[test]
fn size_hint() {
    // TODO:
    // let vec = ts::normal();
    // let target = vec.sparse_reader();
    // let result = target.size_hint();
    // assert_eq!(result, (vec.nnp(), Some(vec.nnp())));
}


#[test]
fn next_back() {
    // TODO:
}