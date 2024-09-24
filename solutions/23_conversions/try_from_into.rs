// `TryFrom`は簡潔で安全な型変換です。
// この変換ではいくつかの状況によっては制御された通りに処理が失敗することがある。
// 基本的には`From`と同じです。
// 主な違いは自身の型を返す代わりに`Result`型を返さなければならない点です。
// `TryFrom`についての詳細は以下のサイトを読みましょう。
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html

#![allow(clippy::useless_vec)]
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// `TryFrom`のエラーとして`IntoColorError`を使います。
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // スライスの長さが不正値のとき
    BadLen,
    // 整数値の変換エラーのとき
    IntConversion,
}

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (Ok(red), Ok(green), Ok(blue)) = (
            u8::try_from(tuple.0),
            u8::try_from(tuple.1),
            u8::try_from(tuple.2),
        ) else {
            return Err(IntoColorError::IntConversion);
        };

        Ok(Self { red, green, blue })
    }
}

impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        // タプルの実装を再利用します。
        Self::try_from((arr[0], arr[1], arr[2]))
    }
}

impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        // 配列の長さを確認します。
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        // タプルの実装を再利用します。
        Self::try_from((slice[0], slice[1], slice[2]))
    }
}

fn main() {
    // `try_from`関数を使ってください。
    let c1 = Color::try_from((183, 65, 14));
    println!("{c1:?}");

    // `TryFrom`が`Color`に実装されているので`TryInto`を使うことができます。
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{c2:?}");

    let v = vec![183, 65, 14];
    //`try_from`関数を使ってください。
    let c3 = Color::try_from(&v[..]);
    println!("{c3:?}");
    // もしくは`try_into`を使って、ブラケット内にスライスを配置してください。
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{c4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use IntoColorError::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(Color::try_from((256, 1000, 10000)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(Color::try_from((-1, -10, -256)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(Color::try_from((-1, 255, 255)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }

    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }
}
