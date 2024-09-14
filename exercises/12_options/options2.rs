fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO:Option型のSomeを値としてもつ`if-let`の構文を作成してください。
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: `while-let`構文に変更してください。
        // また`Vec::pop()`がOption型を返すことを覚えておいてください。
        // `if-let`や`while-let`でも入れ子になったパターンのマッチングを行うことができます。
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
