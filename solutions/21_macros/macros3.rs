// `macro_use` を追加します。
// `macro_use`の補足：https://doc.rust-lang.org/reference/macros-by-example.html#the-macro_use-attribute
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
