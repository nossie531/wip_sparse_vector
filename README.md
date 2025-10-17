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
そのため、この型はイテレータになれない。

この型は逆方向の操作を提供しない。
これも逆方向に戻った時に要素が消えている場合があるためである。

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

## TODO 1

slice の split 系をまねた関数について。

戻り値のために SparseSlice のような名前の型が必要になりそう。
また、split_mut 系の関数では unsafe が必要になるはずだ。
下手に実装するとバグだらけになりそうである。

## TODO 2

IterMut があっても良いのでは？
走査位置さえあればできそう。

## TODO 3

Index で範囲を指定してスライスを取得できても良いのでは？

## TODO 4

size_hint を SparseSlice で予測できないだろうか？
BTreeMap の Range では size_hint があてにならない。

SparseSlice 内の nnp の範囲を [min_nnp, max_nnp] とする。
len を n と r に分割した場合、新たな min_nnp は min_nnp - r、
新たな max_nnp は max_nnp.min(n) となる。

SparseSliceMut は SparseSlice に変換でき、かつ NNP が変化しうる。
そのため、この経路についてはスライス長のみを根拠にすべき。
