// このエクササイズでは`numbers`という`u32`型の0~99までの要素数を持つ配列が与えられます。
// この配列を8個の異なるスレッドで同時並行に利用したい。
// それぞれのスレッドはオフセットの8番目の値の合計値を取得する。
// 例えば、
// 最初のスレッド(オフセット0)：0, 8, 16...の合計
// ２番目のスレッド(オフセット1)：1, 9, 17...の合計
// ３番目のスレッド(オフセット2)：2, 10, 18...の合計
// 8番目のスレッド(オフセット7)：7, , 18...の合計
//
// それぞれのスレッドは参照を累計する配列の数のポインターを所有すべきであるが、
//`Rc`はスレッドとして安全ではない。そこで`Arc`を使う。
//
// このエクササイズではスレッドがどのように生成され、使われるかに気を取られないでください。
// スレッドについては後のエクササイズで扱います。

// この2行は変更しないでください。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    let shared_numbers = Arc::new(numbers);
    //                   ^^^^^^^^^^^^^^^^^

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        //                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
