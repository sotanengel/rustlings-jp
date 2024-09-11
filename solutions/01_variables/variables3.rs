#![allow(clippy::needless_late_init)]

fn main() {
    // Rustでは初期化されていない変数を読み込むことは認められていません。
    // したがって最初に変数に値を代入する必要があります。
    let x: i32 = 42;

    println!("Number {x}");

    // 変数の宣言後に初期化をすることも可能です。
    // しかしその変数を初期化前に使うことはできません。
    let y: i32;
    y = 42;
    println!("Number {y}");
}
