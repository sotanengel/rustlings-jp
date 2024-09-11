fn main() {
    let number = "T-H-R-E-E";
    println!("Spell a number: {}", number);

    // 変数名のシャドーイングを使いましょう。
    // https://doc.rust-jp.rs/book-ja/ch03-01-variables-and-mutability.html#%E3%82%B7%E3%83%A3%E3%83%89%E3%83%BC%E3%82%A4%E3%83%B3%E3%82%B0
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
