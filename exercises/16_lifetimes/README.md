# ライフタイム

ライフタイムはコンパイラーに参照が与えられや状況において十分に有効かどうかをどのように確認するか教えます。
例えばライフタイムは「`a`というパラメータは`b`というパラメータと同じくらい有効であるので、返り値も有効ですよ」と言ったようなことを教えてくれます。

ライフタイムは参照といった借用や譲渡されたパラメータがそのスコープ内でのみ所有権を有しており、外部で参照できなくなるような際にのみ必要である。
ライフタイムとは呼び出しを行う関数が自身の引数が有効であることを確認できるという意味である。
ライフタイムは呼び出しを行うものに制限する。

もしもライフタイムの注釈をさらに学びたいのであれば、[lifetimekata](https://tfpk.github.io/lifetimekata/) プロジェクトがRustlingsに似たエクササイズを作成しています(ただし全てライフタイムに関するものですが)。

# 補足情報

- [Lifetimes (in Rust By Example)](https://doc.rust-jp.rs/rust-by-example-ja/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
