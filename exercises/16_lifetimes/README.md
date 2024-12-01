# ライフタイム

ライフタイムは任意の与えられた状況において、参照が有効になるために十分な長さの生存期間があるかどうかをチェックする手段をコンパイラに伝えます。
例えばライフタイムは「戻り値が有効になるためには、パラメータ`a`とパラメータ`b`が同じ長さの生存期間であるようにしてください。」といったようなことを教えてくれます。

ライフタイムは参照といった借用や譲渡されたパラメータがそのスコープ内でのみ所有権を有しており、外部で参照できなくなるような際にのみ必要です。
ライフタイムとは呼び出しを行う関数が自身の引数が有効であることを確認できるということを意味します。
ライフタイムは呼び出し元を拘束します。

もしもライフタイムをさらに学びたいのであれば、[lifetimekata](https://tfpk.github.io/lifetimekata/) プロジェクトがRustlingsに似たエクササイズを作成しています(ただし全てライフタイムに関するものですが)。

# 補足情報

- [Lifetimes (in Rust By Example)](https://doc.rust-jp.rs/rust-by-example-ja/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
