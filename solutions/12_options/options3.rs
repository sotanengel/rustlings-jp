#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // Solution 1: `Some`の所有権を移動させることがなく`Option`型のマッチングを行う。
    // `&Some`ではなく`ref p`と表記することに注意しましょう。
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        //   ^^^ added
        _ => panic!("No match!"),
    }

    // Solution 2: // Solution 2: `optional_point`に`&`を加えて、参照したものにマッチングを行う。
    match &optional_point {
        Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}");
}
