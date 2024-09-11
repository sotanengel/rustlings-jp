// TODO: 関数の`num`引数に対して欠けている型情報を`:`の後に追加してください。
fn call_me(num:) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
