// ハッシュマップ形式のフルーツのカゴを定義してください。
// キー名をフルーツの名前、バリューはカゴの中のフルーツの数を表します。
// 今回の場合は少なくとも3種類のフルーツを格納し、カゴの中の果物の数は5個以上にしてください。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: ハッシュマップを定義してください。
    // let mut basket =

    // 2つのバナナが既にハッシュマップに追加されています。 :)
    basket.insert(String::from("banana"), 2);

    // TODO: さらに果物を追加してください。

    basket
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
