# 型の変換

Rustはある型を他の方に変える複数の手段がある。

最も簡単な型の変換はキャスト式です。キャスト式はバイナリのオペレータである`as`で示されます。
例えば`println!("{}", 1 + 1.0);`はコンパイルされません。`1`(整数型)と`1.0`(float)で型が異なるからです。
しかし型の変換を行った`println!("{}", 1 as f32 + 1.0)`はコンパイルができます。
[`using_as`](using_as.rs)のエクササイズではこの問題を扱っています。

Rustは実装時に型変換を容易にするトレイトも提供している。
このトレイトは[`convert`](https://doc.rust-lang.org/std/convert/index.html)モジュールなどで見ることができます。
The traits are the following:

- `From` と `Into`を変換する(※ [`from_into`](from_into.rs)で扱います。)
- `TryFrom` と `TryInto`を変換する(※ [`try_from_into`](try_from_into.rs)で扱います。)
- `AsRef` と `AsMut`を変換する(※ [`as_ref_mut`](as_ref_mut.rs)で扱います。)

さらに`std::str`モジュールは[`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)というトレートを提供しており、
このトレートによって`parse`メソッド経由で文字列を狙った方に変換することが容易になる。
もしも`Person`型を導入する場合、`let p: Person = "Mark,20".parse().unwrap()`はコンパイルと実行がパニックなく行われる。


これらの型変換は***within the standard library***で示された一般的な方法でデータをあなたが望む型に変更するべきです。

# 補足情報

型の変換に関しては直接本で対応しているところはないが、Rustのドキュメントが詳しい説明を載せている。

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)
