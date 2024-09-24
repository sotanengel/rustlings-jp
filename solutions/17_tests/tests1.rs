// テストはコードが意図したように動作しているか確認するために重要です。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    // TODO: `is_even`をインポートしてください。外部にあるモジュールをインポートするためにワイルドカードを使うことができます。
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(is_even(0));
        assert!(!is_even(-1));
        //      ^ `!`を前に入れることで`false`を取り扱うことができます。
    }
}
