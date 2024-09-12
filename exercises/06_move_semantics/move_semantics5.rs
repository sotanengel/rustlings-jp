#![allow(clippy::ptr_arg)]

// TODO: 参照(&)をつけたり、削除したりすることでコンパイルエラーを修正してください。
// 所有権を取得しないでください。
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// 所有権を取得してください。
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}
