use std::mem;

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // `Option`型の`unwrap`は`None`に対してはpanicを起こす。
    // そのため `if-let`を代わりに使う。
    if let Some(value) = my_option {
        println!("{value:?}");
    }

    // コンマが入れ忘れになっていた。
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // `resize`は新しい配列を返す代わりに配列自身を変更する、
    // `resize(0, …)`は配列を初期化するので、`clear`を代わりに使った方が良い。
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // スワップを正確に行うために`mem::swap`を使う。
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
