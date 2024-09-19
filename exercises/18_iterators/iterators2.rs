// このエクササイズではイテレータ特有の有利な点をいくつか学びます。

// TODO: Complete the `capitalize_first`関数を完成させてください。
// "hello"が入力された場合に"Hello"を返します。
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => todo!(),
    }
}

// TODO:`capitalize_first`関数を文字列のスライスに対して実行する関数を書いてください。
// 返り値は文字列の配列を返してください。
// ["hello", "world"]が入力された場合に["Hello", "World"]を返します。
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
}

// TODO: `capitalize_first`関数をスライスの文字列に対して実行する関数を書いてください。
// 返り値は一つの文字列で返してください。
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
}

fn main() {
   // この行で関数のテストができます。 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
