fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // 解答1：`strings2`を内部のブロックから出すことで出力が実行される前に所有権が剥奪されない。
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
    //  `string2` は関数の最後に所有権を失う。

    // =========================================================================

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // 解答2：出力を行う一文を内部のブロックに移動させることで、`string2`の所有権が失われる前に実行されるようにした。
        println!("The longest string is '{result}'");
        // `string2`は内部のスコープの最後に所有権を失う。
    }
}
