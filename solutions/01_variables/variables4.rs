fn main() {
    // Rustでは変数は標準で不変です。
    // `mut`キーワードを`let`の後につけることで可変の変数であると宣言することができます。
    let mut x = 3;
    println!("Number {x}");

    x = 5;
    println!("Number {x}");
}
