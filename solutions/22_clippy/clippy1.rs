// このエクササイズではClippyの警告がある場合にはコードのコンパイルが実行できないようです。
// Clippyの提案を確認しエクササイズがクリアできるように修正してください。

use std::f32::consts::PI;

fn main() {
    // より正確な定数である`PI`を利用しましょう。
    let pi = PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
