// AsRefとAsMutは参照と参照の変換を容易に行うことができる。
// 詳細については https://doc.rust-lang.org/std/convert/trait.AsRef.html と
// https://doc.rust-lang.org/std/convert/trait.AsMut.html を読みましょう。

// 引数からバイト数を取得しましょう。
// TODO: トレイト境界として`AsRef`を追加しましょう。
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// // 引数から文字数を取得しましょう。
// TODO: トレイト境界として`AsRef`を追加しましょう。
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// `as_mut()`を使って数値を二乗しましょう。
// TODO: 適切なトレイト境界を追加しましょう。
fn num_sq<T>(arg: &mut T) {
    // TODO: 関数の内部を実装しましょう。
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
