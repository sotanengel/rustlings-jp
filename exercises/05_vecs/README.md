# ベクタ

ベクタはRustで最も使われているデータ構造のひとつです。
他のプログラミング言語では単純に配列と呼ばれているものですが、Rustでは少し低レベルで動作するため配列はスタックに格納されます(つまり配列の要素数は増減せず、コンパイル時に配列サイズがわかっている必要があります)。
一方ベクタはヒープに格納されます(上記のような制限はありません)。

ベクタはthe bookではここよりも少し後ろの章で扱っていますが、ベクタはRustを学ぶ上で十分有用であると思うため、このエクササイズでは早めに取り扱うことにしました。その他の有用なデータ構造であるハッシュやマップについて後ほど取り扱います。

# 補足情報

- [Storing Lists of Values with Vectors](https://doc.rust-jp.rs/book-ja/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
