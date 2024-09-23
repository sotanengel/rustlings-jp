// このエクササイズではClippyの警告がある場合にはコードのコンパイルが実行できないようです。
// Clippyの提案を確認しエクササイズがクリアできるように修正してください。

fn main() {
    // TODO: Clippyからの修正をここで実行してください。
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
