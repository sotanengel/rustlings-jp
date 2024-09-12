fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: `vec0`と`vec1`ベクタに同じタイミングでアクセスできるように
    // テスト内のコンパイルエラーを修正してください。
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
