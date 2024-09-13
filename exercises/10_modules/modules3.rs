// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.
// `use`キーワードは標準のライブラリのモジュールなど、どこにあるモジュールでもスコープに入れることができます。

// TODO: `SystemTime`や`UNIX_EPOCH`を`std::time`モジュールからスコープに追加してください。
// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
