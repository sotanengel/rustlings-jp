fn trim_me(input: &str) -> &str {
    // TODO: 文字列の前後の空白を取り除いてください。
}

fn compose_me(input: &str) -> String {
    // TODO: 文字列の末尾に" world!"を追加してください。
    // いくつかの記載方法があります。
}

fn replace_me(input: &str) -> String {
    // TODO: "cars"という文字列を"balloons"に置換してください。
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
