// これは以下のセクションのクイズです：
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// 関数形式の機械を作りましょう。この機械への入力は文字列とコマンドの組み合わせでベクターで渡します。
// これらのコマンドは以下のような文字列への処理を決定するものです。
// - 文字列を大文字にする
// - 文字列をトリミングする
// - 特定回数の"bar"を追加する
//
// より詳細な仕様は以下です：
// - 入力は要素数が2のタプルで最初の要素が変更を加える文字列で2つ目がコマンドです
// - 出力は文字列のベクターです

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: 関数を以下に完成させてください。
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    // TODO: `transformer`をスコープに入れるために何をすればいいですか？
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
