// ライフタイムは構造体が参照を持つ際にも必要になります。

// TODO: 構造体に関するコンパイルエラーを修正してください。
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
