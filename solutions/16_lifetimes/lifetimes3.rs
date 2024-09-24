// ライフタイムは構造体が参照を持つ際にも必要になります。

struct Book<'a> {
    //     ^^^^ ライフタイムの注釈を追加しました。
    author: &'a str,
    //       ^^
    title: &'a str,
    //      ^^
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
