fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: `v`というベクタを作成し、`a`と同じ要素を格納してください。
    // ベクタのマクロを使ってください。
    // let v = ???;

    (a, v)
}

fn main() {
    // この行でarray_and_vec関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
