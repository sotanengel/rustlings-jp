fn main() {
    let mut res = 42;
    let option = Some(12);
    // イテレーターの代わりに`if-let`を使いましょう。
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
