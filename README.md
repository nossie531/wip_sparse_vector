# sparse_vector (WIP)

Vector like type for sparse values.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

This crate provides a vector like type `SparseVec`.
This type is efficient when most elements have same value.

## Examples

```rust
let mut v = SparseVec::new(5);
*v.edit(0) = 1;
*v.edit(2) = 3;
*v.edit(4) = 5;

assert_eq!(v.to_vec(), vec![1, 0, 3, 0, 5]);

for (_idx, val) in v.sparse_writer() {
    *val += 1;
}

assert_eq!(v.to_vec(), vec![2, 0, 4, 0, 6]);
```

## ❌ Buggy!!

SparseWriter を破棄したあともそこから得た参照を編集できる。
これによりドロップ処理が空ぶる。
対策1: 借用イテレータにする？
対策2: ValueEditor のような値を生成する？

## Future task 1

以下の実装はどれも `T` が制約されすぎている。

- `impl<T: PartialEq + Clone> Clone for Iter<T>`
- `impl<T: PartialEq + Clone> Clone for SparseReader<'_, T>`

これらは解決可能だが、現状では成果とコード量のバランスが悪いため放置している。
将来的には、以下のどれかで解決する予定。

- Rust が機能 [`btree_cursors`] を正式採用する。
- [`pstd`] クレートで私の[プルリク][my_pr]が承認される。
- 私が同等の機能を自前で実装する。

[`btree_cursors`]: https://doc.rust-lang.org/beta/unstable-book/library-features/btree-cursors.html
[`pstd`]: https://crates.io/crates/pstd
[my_pr]: https://github.com/georgebarwood/pstd/pull/2

## MEMO 1

Index で範囲を指定してスライスを取得できても良いのでは？
無理！Index の結果は参照なので SparseSlice を
どこかに配置しておかないといけない。

## MEMO 2

IterMut があっても良いのでは？
無理！パディング値から通常値にした場合の保存先がない。
もし実装するなら借用イテレータ形式でないと。

## TODO 1

SparseSliceMut の可変分割について。以下が必要。
- slice_mut 
- split_mut
後者は一つの値を二つの可変参照で覗くため unsafe は不可避。
高次元による分割も考えるとより複雑に…。
SparseSliceMut から SparseVec へはポインタでの接続に変えるべきかも。
だけど 二つの SparseSliceMut から SparseWriter を作ると、
結局 MapRangeMut が二つになっちゃう…。
MapRangeMut の代わりに MapRange を使って削除や登録は別経路にすべきかも。

## TODO 2

SparseVec::drain も実装すべき。
SparseVec::splice の亜種なので簡単に実装できるはず。
SparseVec::erase でパディングで埋めれてもいいかも。

## TODO 3

SparseSlice と SparseSliceMut は互いに比較できるべきでは？
