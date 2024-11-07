// `From`トレイトは値の変換に用いられる。
// `From`が実装された場合、`Into`が自動的に提供される。
// このトレイトについては以下のドキュメントでさらに詳細を知ることができる。
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 提供されている文字列が`Person`オブジェクトに変換できない時、デフォルトのトレイトを実装して代替として使う。
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO:  "Mark,20"という形式の文字列から`Person`型にパースできるように以下の`From`実装を完成させてください。
// `"4".parse::<u8>()`のように年齢要素は`u8`型にパースする必要があります。
// 実装の手順:
// 1. 文字列をカンマで分割してください。
// 2. 分割処理の結果として2個よりも大きいもしくは小さい場合は、デフォルトの`Person`を返してください。
// 3. 分割した要素のうち、最初の要素は名前として使ってください。
// 4. 名前の部分が空白だった場合には、デフォルトの`Person`を返してください。
// 5. ２つ目の要素は年齢として`u8`型にしてください。
// 6. もしも年齢のパースに失敗した場合には、デフォルトの`Person`を返してください。
impl From<&str> for Person {
    fn from(s: &str) -> Self {}
}

fn main() {
    // `from`を使ってください。
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // `From`が`Person`で実装されたので、`Into`を使うことができます。
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
