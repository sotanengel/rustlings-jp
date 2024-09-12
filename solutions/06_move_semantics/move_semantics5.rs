#![allow(clippy::ptr_arg)]

// 所有権を取得する代わりに借用しましょう。
// この関数では`&String`の代わりに`&str`を使うことが好ましいですが、
// ここではstring型を取り扱わないのでこのままで大丈夫です。
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 借用の代わりに所有権を取得しましょう。
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
