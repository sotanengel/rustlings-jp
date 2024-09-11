// このお店では価格が偶数の時に10の割引を受けることができますが、奇数の時には3の割引を受けることができます。
// 関数の中身については今回は気にしなくとも大丈夫です。

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: 関数の入出力の記載内容を修正してください。
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
