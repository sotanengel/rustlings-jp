fn main() {
    my_macro!();
}

// TODO: マクロの定義の位置を修正してコンパイルエラーを修正してください。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
