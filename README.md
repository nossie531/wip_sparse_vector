# sparse_vector

作成中。

## Future task 1

以下の実装はどれも `T` が制約されすぎている。

- `impl<T: PartialEq + Debug> Debug for IntoIter<T>`
- `impl<T: PartialEq + Clone> Clone for Iter<T>`
- `impl<T: PartialEq + Clone> Clone for SparseReader<'_, T>`

これらは解決可能だが、現状では成果とコード量のバランスが悪いため放置している。
将来的には、Rust が機能 [`btree_cursors`] を正式採用するか、
[`pstd`] クレートで私の[プルリク][my_pr]が承認されるか、
私が同等の機能を自前で実装すれば解決される。

[`btree_cursors`]: https://doc.rust-lang.org/beta/unstable-book/library-features/btree-cursors.html
[`pstd`]: https://crates.io/crates/pstd
[my_pr]: https://github.com/georgebarwood/pstd/pull/2

## TODO

slice の split 系をまねた関数について。

戻り値のために SparseSlice のような名前の型が必要になりそう。
また、split_mut 系の関数では unsafe が必要になるはずだ。
下手に実装するとバグだらけになりそうである。
