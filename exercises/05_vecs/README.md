# ベクタ

ベクタはRustで最も使われているデータ構造です。
他のプログラミング言語ではアレイ型と呼ばれていますが、Rustでは少し低いレベルで動作するため、
Rustでの配列はスタックに格納されます(つまり配列の要素数は増減せず、コンパイル時に配列サイズがわかっている必要があります)。
一方ベクタはヒープに格納されます(上記のような制限はありません)。

ベクタは書籍では後の章で扱いますが、ベクタはRustを学ぶ上で十分有用であると思うため、早く取り扱うことにした。
この問題集ではその他の有用なデータ構造であるハッシュやマップについて後ほど取り扱う。

# 補足情報

- [Storing Lists of Values with Vectors](https://doc.rust-jp.rs/book-ja/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
