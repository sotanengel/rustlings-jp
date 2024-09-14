// トークンを使ってアイテムを購入するゲームをコーディングします。
// 全てのアイテムは5トークンで買うことができますが、購入のたびに利用料として1トークンがかかります。
// ゲームのプレイヤーは購入したいアイテムの個数を入力することができ、`total_cost`関数は必要なトークン数を計算します。
// プレイヤーが入力する個数をプログラムでは文字列として受け取ります。ただしプレイヤーは数字以外を入力する可能性があります。
// 現状、この関数は全てのエラーケースについてハンドリングできていません。
//　このエクササイズでは、`total_cost`関数で受け取った文字列が数字でなければParseIntErrorを返却したい。
// エラーの場合にはコストの計算を行う前にすぐにエラーを返したい。
//
// 今回は少なくとも2つの書き方があるが、書く分量がかなり異なる。

use std::num::ParseIntError;

#[allow(unused_variables)]
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // `?` を記載することでエラーを伝達することができます。
    let qty = item_quantity.parse::<i32>()?;
    //                                    ^ added

    // これは冗長な書き方です。
    let qty = match item_quantity.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
