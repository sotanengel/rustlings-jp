fn current_favorite_color() -> String {
    // `String::from("blue")`と同じ意味です。
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
