// The type of function arguments must be annotated.関数の引数の型は必ず注釈が必要です。
// `u64`.`u64`の型を追加しましょう。
fn call_me(num: u64) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
