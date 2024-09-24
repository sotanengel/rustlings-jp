fn square(num: i32) -> i32 {
    // 以下の式の`;`を削除することで(暗黙的に)結果をリターンしてください。
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
