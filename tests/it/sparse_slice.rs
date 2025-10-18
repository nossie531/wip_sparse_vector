use crate::for_test::sample as ts;
use crate::for_test::template as tt;

#[test]
fn is_empty() {
    with_zero_len();
    with_some_len();

    fn with_zero_len() {
        let vec = ts::default();
        let target = vec.slice(0..0);
        let result = target.is_empty();
        assert_eq!(result, true);
    }

    fn with_some_len() {
        let vec = ts::normal();
        let range = ts::range(vec.len());
        let target = vec.slice(range);
        let result = target.is_empty();
        assert_eq!(result, false);
    }
}

#[test]
fn len() {
    let vec = ts::normal();
    let range = ts::range(vec.len());
    let target = vec.slice(range.clone());
    let result = target.len();
    assert_eq!(result, range.len());
}

#[test]
fn iter() {
    let template = tt::template();
    let vec = ts::normal();
    let range = ts::range(vec.len());
    let target = vec.slice(range.clone());
    let result = target.iter();
    assert!(result.eq(template.sample_vec()[range].iter()));
}

#[test]
fn sparse_reader() {
    // Arrange.
    let template = tt::template();
    let vec = ts::normal();
    let range = ts::range(vec.len());
    let target = vec.slice(range.clone());

    // Act.
    let result = target.sparse_reader();

    // Assert.
    let lhs = result.map(|e| (e.index(), *e.value()));
    let elms = template.sample_vec().into_iter();
    let elms = elms.skip(range.start).take(range.len()).enumerate();
    let rhs = elms.filter(|e| e.1 != template.padding());
    assert!(lhs.eq(rhs));
}

#[test]
fn to_vec() {
    let template = tt::template();
    let vec = ts::normal();
    let range = ts::range(vec.len());
    let target = vec.slice(range.clone());
    let result = target.to_vec();
    assert_eq!(result, template.sample_vec()[range]);
}

#[test]
fn slice() {
    let template = tt::template().set_len(100);
    let vec = template.build();
    let w_range = ts::range(vec.len());
    let n_range = ts::range(w_range.len());
    let target = vec.slice(w_range.clone());
    let result = target.slice(n_range.clone());
    assert_eq!(result.to_vec(), template.sample_vec()[w_range][n_range]);

    // TODO: 範囲チェック
}