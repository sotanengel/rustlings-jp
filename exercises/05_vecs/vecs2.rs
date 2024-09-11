fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // `input`スライスの各要素に関して2をかけて、`output`ベクタに追加してください。
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // map後のベクタを集める場合の例
    // 受け取った`input`スライスの各要素に1を加え、その値に1を加算した値に対応づける
    // `input`が[1, 2, 3]であれば、出力は[2, 3, 4]。
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: `input`スライスの各要素に2をかけたい(空の配列にpushするのではなく、mapを使って)。
    // `vec_map_example`関数を参考にしてください。
    input
        .iter()
        .map(|element| {
            // ???
        })
        .collect()
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
