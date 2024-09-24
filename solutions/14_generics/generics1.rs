// `Vec<T>`は`T`型のジェネリクスである。ほとんどの場合、コンパイラーは`T`の型を推論することができます。
// 例えば配列に固定された型の値を追加した後などである。
// しかしこのエクササイズではコンパイラーは型の注釈が必要です。

fn main() {
    // `u8`や`i8`を内包する`i16`に変更しました。
    let mut numbers: Vec<i16> = Vec::new();
    //             ^^^^^^^^^^ added

    // これ以降は変更しなくてください。
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
