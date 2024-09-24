fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    // `.into()`は型をコンパイラが推定した型に変換する。
    // `String`とコンパイラが推定する場合には、`&str`が`String`に変換されます。
    string("nice weather".into());
    // ただし`&str`と推定される場合には`&str`のままとなる。
    // もしも`#[allow(…)]`の行を削除した場合には、Clippyが`.into()`を削除することを提案します。
    #[allow(clippy::useless_conversion)]
    string_slice("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: 文字ごとのインデックスではなくバイトごとのインデックスです。
    // 文字のインデックスを利用するためには`s.chars().nth(INDEX)`と記載します。
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
