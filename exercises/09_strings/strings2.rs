// TODO: この関数を変えることなくmain関数内部を変更することで、
// コンパイルエラーを修正してください。
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // この行は変えないでください。

    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
