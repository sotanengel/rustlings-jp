// `AppendBar`トレイトは、このトレイトを移植したいかなるオブジェクトに対して"Bar"という文字列を末尾に追加する
//　関数のみを持ちます。
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO:`AppendBar`トレイトを`String`型に移植してください。
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
