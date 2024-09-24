// Rustのコンパイラーは与えられた参照が有効かどうかをどのように確認すればいいか知る必要があります。
// そのためプログラマーは参照が使われる前にスコープからもしも外れるリスクを把握する必要があります。
// 参照は借り物であり、それ自身に情報を保有していないことを留意してください。
// 所有権がどのようにしてスコープから外れたらどうなるのでしょう？

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //    ^^^^     ^^          ^^          ^^
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
