fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(2 * element);
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
    // イテレータをこれから深く学んでいくことになるが、ここではこの回答で十分である。
    // 高度な解説: このメソッドは十分なリソースを事前に割り振るため、効率的に処理が行われる、
    // このメソッドでは`Vec::new()`の代わりに`Vec::with_capacity(input.len())`を用いて、
    // `vec_loop`で実行される。
    input.iter().map(|element| 2 * element).collect()
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
