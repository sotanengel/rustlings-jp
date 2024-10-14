fn factorial(num: u64) -> u64 {
    // TODO: 以下のような階乗を実行する関数`num`を完成させてください。
    // `1 * 2 * 3 * … * num`。
    // https://ja.wikipedia.org/wiki/%E9%9A%8E%E4%B9%97
    //
    // 以下の方法は禁止です:
    // - 早期にreturnを使う (明示的にreturnを使う) 
    // 以下の方法はできるだけ使わないでください:
    // - ループ (for/while)
    // - 変数を追加する
    // 挑戦問題、できれば以下を使わないでください:
    // - 再起
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
