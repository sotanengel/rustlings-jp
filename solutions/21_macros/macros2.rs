// マクロの定義する位置を呼び出す場所の前に移動しました。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
