// この関数を呼び出す代わりに、`string_slice`か`string`をmain関数内で呼び出してください。
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 以下に記載された文字列は`String`か`&str`が該当します。
// `placeholder(…)`を`string_slice(…)`か`string(…)`に置き換えてください。
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

  // WARNING: 文字ごとのインデックスではなくバイトごとのインデックスです。
  // 文字のインデックスを利用するためには`s.chars().nth(INDEX)`と記載します。
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
