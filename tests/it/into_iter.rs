use crate::for_test::template as tt;
use sparse_vector::IntoIter;

#[test]
fn default() {
    let result = IntoIter::<i32>::default();
    assert_eq!(result.size_hint(), (0, Some(0)));
    assert_eq!(result.count(), 0);
}

#[test]
fn size_hint() {
    let template = tt::template();
    let vec = template.build();
    let target = vec.into_iter();
    let result = target.size_hint();
    assert_eq!(result, (template.len(), Some(template.len())));
}

#[test]
fn next() {
    // todo.
}
