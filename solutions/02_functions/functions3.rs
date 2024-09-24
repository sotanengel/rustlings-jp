fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // `call_me`は引数の型を推測します。
    call_me(5);
}
