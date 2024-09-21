// コンパイルする時、Rustは型がどれだけの領域を使うか知る必要がある。これは再帰データ型の取り扱いに問題を引き起こす。
// なぜならその値が他の値の一部になる可能性があるからだ。
// この問題を取り扱うために、`Box`というデータをヒープ上に保管するために使われるスマートポインタを利用することができる。
// このスマートポインタにより再帰データ型をラップするができるようになる。
// このエクササイズで実装する再帰データ型は「"cons list"」である。
// このリストは関数型プログラミング言語でよく見るデータ構造である。
// リスト内のそれぞれの要素は２つの属性を含んでいる。現在の要素と次の要素である。最後の要素は次の要素として`Nill`を呼び出す。

// TODO: コードがコンパイルできるようにenumの定義の中で`Box`を利用してください。
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: 空のconsリスト(Nil)を作成してください。
fn create_empty_list() -> List {
    todo!()
}

// TODO:空ではないconsリスト(どれだけ再帰するかは自身で決めて大丈夫です)を作成してください。例. (42, (35, Nil))など
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
