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

## TODO 0

CURRENT LOCATION:
SparseSlice の頭にプロローグを追加するのに splice が必要に…。
またより高速なパディング専用版に splice_paddings も実装中。

Vec::splice 相当のメソッドについて。
SparseVec::splice はそれと微妙に動作が異なる。
(デストラクタを抑制しないと検出できない程度の差異だが…。)

Vec::splice は呼出時に削除とその内容のバックアップ (戻り値のイテレータ用) を、
戻り値の破棄時に登録を行う。

一方、SparseVec::splice は呼出時は何もせず、
戻り値の破棄時に全ての削除と登録を行う。

これは背後に B 木を使用しているため、バックアップ時にマップが分割されて
そのオーバーヘッドによりヒープが増えないようにしている (完全な削除で
ヒープが足りなくなるのは避けたいので)。

## TODO 1

SparseSlice の可変分割について。
slice::split_mut と同じ使用感を目指す。
一つの値を二つの可変参照で覗くため unsafe は不可避。
`Rc<RefCell<Map>>` のようにマップのラップが必要。

## TODO 2

IterMut があっても良いのでは？
走査位置さえあればできそう。

## TODO 3

size_hint を SparseSlice で予測できないだろうか？
BTreeMap の Range では size_hint があてにならない。

SparseSlice 内の nnp の範囲を [min_nnp, max_nnp] とする。
len を n と r に分割した場合、新たな min_nnp は min_nnp - r、
新たな max_nnp は max_nnp.min(n) となる。

SparseSliceMut は SparseSlice に変換でき、かつ NNP が変化しうる。
そのため、この経路についてはスライス長のみを根拠にすべき。
