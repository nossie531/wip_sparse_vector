# sparse_vector

作成中。

## SparseReader (iterator)

`SparseReader` は疎な要素を走査しながら読み込むための型である。
この型は `SparseVec::sprase_reader` メソッドから取得できる。

## SparseWriter (not iterator)

`SparseWriter` は疎な要素を走査しながら書き込むための型である。 
この型は `SparseVec::sparse_writer` メソッドから取得できる。

この型はイテレータではない。
なぜなら、イテレータは走査途中での要素の挿入や削除を認めない。
一方、この型は走査中に要素をパディング値にできる。
パディング値にした要素は再走査で見つからないので、これは削除と同義である。
つまり、この型はイテレータになれない。

## Future task 1

以下の実装はどれも `T` が制約されすぎている。

- `impl<T: PartialEq + Clone> Clone for Iter<T>`
- `impl<T: PartialEq + Clone> Clone for SparseReader<'_, T>`

これらは解決可能だが、現状では成果とコード量のバランスが悪いため放置している。
将来的には、Rust が機能 [`btree_cursors`] を正式採用するか、
[`pstd`] クレートで私の[プルリク][my_pr]が承認されるか、
私が同等の機能を自前で実装すれば解決される。

[`btree_cursors`]: https://doc.rust-lang.org/beta/unstable-book/library-features/btree-cursors.html
[`pstd`]: https://crates.io/crates/pstd
[my_pr]: https://github.com/georgebarwood/pstd/pull/2

## TODO 1

slice の split 系をまねた関数について。

戻り値のために SparseSlice のような名前の型が必要になりそう。
また、split_mut 系の関数では unsafe が必要になるはずだ。
下手に実装するとバグだらけになりそうである。

## TODO 2

IterMut があっても良いのでは？
走査位置さえあればできそう。
