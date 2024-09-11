fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: `call_me`関数の呼び出し方を修正してください。
    call_me();
}
