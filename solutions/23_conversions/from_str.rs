// このエクササイズは以前の`from_into`に似ている。
// しかし今回の場合は、`FromStr`を実装し、エラーを返却する代わりにデフォルトの値を返却する。
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

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let (Some(name), Some(age), None) = (split.next(), split.next(), split.next()) else {
            //                      ^^^^ ３つ目の要素は存在してはならない
            return Err(ParsePersonError::BadLen);
        };

        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age = age.parse().map_err(ParsePersonError::ParseInt)?;

        Ok(Self {
            name: name.into(),
            age,
        })
    }
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
