fn bigger(a: i32, b: i32) -> i32 {
    // TODO: この関数は引数として受け取ったaとbのうち、大きい方をリターンします。
    // もしも同じ数字であればどちらかの引数を返却します。
    // 以下を使ってはいけません：
    // - 他の関数を呼び出す
    // - 他の変数を追加する
}

fn main() {
    // この行でbigger関数のテストができます。
}

// 以下は気にしなくとも結構です。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
