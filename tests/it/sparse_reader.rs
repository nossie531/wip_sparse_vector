use crate::for_test::sample as ts;
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
