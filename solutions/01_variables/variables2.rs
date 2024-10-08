fn main() {
    // 一番簡単なコンパイルエラーの修正方法はxに値を代入して、初期化することです。
    // xに整数を設定するとRustは整数の標準の型である`i32`と推測します。
    let x = 42;

    // しかし`i32`ではなく、他の型を強制することも可能です。
    // let x: u8 = 42;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
