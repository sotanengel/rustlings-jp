fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    //  ^^^ added

    vec.push(88);

    vec
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // `fill_vec`ベクタに所有権を譲渡したため、`vec0`ベクタにはこれ以降アクセスできなくなります。
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
