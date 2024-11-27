# Options

Option型はオプションの値を表します。つまり、すべてのOptionは値を含むSome、もしくは値の含まないNoneのどちらかです。
Option型は様々な使い方ができるため、Rustではよく使われます。
使い方の例としては、
- 初期値
- 関数の返り値；特に全ての入力値を定義できない場合(partial functions)。
- 値を返す、あるいはNoneであればエラーを返すといったような簡潔なエラーを返す
- 構造体のフィールドをOptionalにする
- 借用や譲渡ができる構造体のフィールド
- Optionalな関数の引数
- Nullを取りうるポインター
- 複雑な状況から脱出する

# 補足情報

- [Option Enum Format](https://doc.rust-jp.rs/book-ja/ch10-01-syntax.html#enum%E5%AE%9A%E7%BE%A9%E3%81%A7%E3%81%AF)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-jp.rs/rust-by-example-ja/flow_control/if_let.html)
- [while let](https://doc.rust-jp.rs/rust-by-example-ja/flow_control/while_let.html)
