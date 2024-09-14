// これはerrors4のエクササイズを回収したものです。
// ここでは後のエクササイズで学ぶ`Box`や`From`などの見慣れない表記もありますが、今すぐ理解する必要はありません。
// 現時点では`Box<dyn ???>`型は「???をする何かが欲しい」と理解していればいいです。

// 端的にいうと、`Box`を使うのは特定の値や特定の型を持つことを気にする場合である。
// そのために`Box`は`Box<dyn Trait>`型として宣言され、そのトレートはコンパイラがその文脈で使用される値に対して検索する trait である。
// このエクササイズにおける文脈は、Resultとして返すエラーとして取りうるものである(つまり負の値やゼロ)。

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
