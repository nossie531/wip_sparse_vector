use sparse_vector::Iter;

#[test]
fn default() {
    let result = Iter::<i32>::default();
    assert_eq!(result.size_hint(), (0, Some(0)));
    assert_eq!(result.count(), 0);
}
