# sparse_vector (WIP)

A vector for sparse elements.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

This crate provides a vector like type `SparseVec`.
This type is efficient when most elements have same value.

## Examples

```rust
// TODO:
```

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
これは無理！Index の結果は参照なので SparseSlice を
どこかに配置しておかないといけない。

## TODO 1

SparseSliceMut の可変分割について。以下が必要。
- slice_mut 
- split_mut
後者は一つの値を二つの可変参照で覗くため unsafe は不可避。
`Rc<RefCell<Map>>` のようにマップのラップが必要。

## TODO 2

IterMut があっても良いのでは？
走査位置さえあればできそう。

## TODO 3

SparseVec::drain も実装すべき。
SparseVec::splice の亜種なので簡単に実装できるはず。

## TODO 4

SparseSlice と SparseSliceMut は互いに比較できるべきでは？
