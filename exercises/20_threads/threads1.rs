// このスレッドは250ms内にそれぞれが実行される複数のスレッドを持っている。
// そして、それぞれのスレッドは実行完了までにどれだけの時間がかかったかをリターンする。
// このプログラムは全てのスレッドが完了するまで持続する必要があり、実行結果を配列に格納する必要がある。

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: 全てのスレッドから得られた結果を`results`配列に収集してください。ここでは`thread::spawn`を返り値としてもつ`JoinHandle`構造体を利用してください。
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
