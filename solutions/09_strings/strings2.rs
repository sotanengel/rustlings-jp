fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green");

    if is_a_color_word(&word) {
        //             ^ コンパイラーによって`&String`型は`&str`型に強制されます。
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
