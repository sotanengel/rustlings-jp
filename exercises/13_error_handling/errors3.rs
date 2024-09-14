// このコードは以前のエクササイズの`total_cost`関数の完成版を使おうとしています。
// しかしうまく動きません。どのように修正すればいいでしょうか？

use std::num::ParseIntError;

// この関数を編集してはいけません。
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: `main`関数を修正することでコンパイルエラーを修正してください。
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
}
