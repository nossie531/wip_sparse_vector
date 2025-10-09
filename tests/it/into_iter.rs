use crate::for_test::template as ts;
use sparse_vec::IntoIter;

#[test]
fn default() {
    let result = IntoIter::<i32>::default();
    assert_eq!(result.size_hint(), (0, Some(0)));
    assert_eq!(result.count(), 0);
}

#[test]
fn size_hint() {
    let vec = ts::template().build();
    let target = vec.into_iter();
    assert_eq!(
        target.size_hint(),
        (ts::template().len(), Some(ts::template().len()))
    );
}

#[test]
fn next() {
    /* TODO: これらをサンプル配列にして…
    - LEN: 0 要素を用意, next
    - LEN: 1 要素を用意, next, next
    - LEN: MANY, NP 0 要素を用意, next
    - LEN: MANY, NP 1 要素を先頭に用意, next, next
    - LEN: MANY, NP 1 要素を中央に用意, next, next
    - LEN: MANY, NP 1 要素を末尾に用意, next, next
    - LEN: MANY, NP 1 要素を先頭に用意, next_back, next, next
    - LEN: MANY, NP 1 要素を中央に用意, next_back, next, next
    - LEN: MANY, NP 1 要素を末尾に用意, next_back, next, next
    - ランダムな複数要素を用意, ランダムに next_back と next を要素がつきるまで反復、対照群と比較。
     */
}
