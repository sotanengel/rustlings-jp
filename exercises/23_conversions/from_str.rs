// このエクササイズは以前の`from_into`に似ている。
// しかし今回の場合は、`FromStr`を実装し、デフォルトの値を返却する代わりにエラーを返却する。
// さらに`FromStr`の実装では`parse`メソッドを実装型のオブジェクトを生成するために使うことができる。
// 以下のドキュメントで`FromStr`の詳細について読むことができます。
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// `FromStr`の実装としてこのエラー型を使うことができます。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // フィールドが不正な数値のとき
    BadLen,
    // 名前フィールドが存在しないとき
    NoName,
    // parse::<u8>()がエラーのとき
    ParseInt(ParseIntError),
}

// TODO: "Mark,20"という形式の文字列から`Person`型にパースできるように以下の`From`実装を完成させてください。
// `"4".parse::<u8>()`のように年齢要素は`u8`型にパースする必要があります。
//
// Steps:
// 1. 文字列をカンマで分割してください。
// 2. 分割処理の結果として2個以上(もしくは以下)だった場合は、`ParsePersonError::BadLen`エラーを返してください。
// 3. 分割した要素のうち、最初の要素は名前として使ってください。
// 4. 名前の部分が空白だった場合には、`ParsePersonError::NoName`エラーを返してください。
// 5. ２つ目の要素は年齢として`u8`型にしてください。
// 6. もしも年齢のパースに失敗した場合には、`ParsePersonError::ParseInt`エラーを返してください。
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
