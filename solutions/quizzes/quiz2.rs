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

    // ループを使って解決しましょう。イテレータを使った`transformer_iter`も確認してみましょう。
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::new();

        for (string, command) in input {
            // 新しい文字列を作ります
            let new_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            };

            // 出力するベクターに変換後の文字列をpushします。
            output.push(new_string);
        }

        output
    }

    // `transform`関数と同等なイテレータを使った書き方です。
    pub fn transformer_iter(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(string, command)| match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            })
            .collect()
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    // `transformer`をインポートします。
    use super::my_module::transformer;

    use super::my_module::transformer_iter;
    use super::Command;

    #[test]
    fn it_works() {
        for transformer in [transformer, transformer_iter] {
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
}
