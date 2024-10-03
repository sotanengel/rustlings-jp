// `Box<dyn Error>`のような全てのエラーをキャッチする型は、コンテンツのエラーを出力したり、
//さらにエラーを伝播する代わりにエラーに基づいて決断を下したいライブラリに対して利用するのは好ましくない。
// そこでカスタムのエラー型を定義し、関数を呼び出した側が次に何をすべきか、関数がいつエラーを返すべきかを決められるようにします。

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// カスタムエラーとして`PositiveNonzeroInteger::parse`を使います。
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: 他のエラー処理を書いてください。
    // fn from_parse_int(???) -> Self { ??? }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // TODO: `parse()`でpanicエラーを返してしまう前に適切なエラーを返すように修正してください。
        let x: i64 = s.parse().unwrap();
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
