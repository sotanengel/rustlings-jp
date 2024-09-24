// 3通りの解答を紹介します。

// `for`ループと可変な変数を使います。
fn factorial_for(num: u64) -> u64 {
    let mut result = 1;

    for x in 2..=num {
        result *= x;
    }

    result
}

// 上記の`factorial_for`と同等かつ短く、`for`ループと可変な変数を使わずに書いた場合。
fn factorial_fold(num: u64) -> u64 {
    // num==0の場合: 2..=0は空となる。
    //              -> `fold`の返す値は1となる。
    // num==1の場合: 2..=1は同様に空となる。
    //              -> `fold`の返す初期値は1となる。
    // num==2の場合: 2..=2 は1つの要素を持つ。
    //              -> 最初の1の値は2とかけられ、その後返却される。
    // num==3の場合: 2..=3 は2つの要素を持つ。
    //              -> 1 * 2が計算されたのち、2 は２つ目の要素の3とかけられ、 6が返却される。
    // 以下同様
    #[allow(clippy::unnecessary_fold)]
    (2..=num).fold(1, |acc, x| acc * x)
}

// `factorial_fold`と同様のClippyに提案される実装済みの関数。
fn factorial_product(num: u64) -> u64 {
    (2..=num).product()
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial_for(0), 1);
        assert_eq!(factorial_fold(0), 1);
        assert_eq!(factorial_product(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial_for(1), 1);
        assert_eq!(factorial_fold(1), 1);
        assert_eq!(factorial_product(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial_for(2), 2);
        assert_eq!(factorial_fold(2), 2);
        assert_eq!(factorial_product(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial_for(4), 24);
        assert_eq!(factorial_fold(4), 24);
        assert_eq!(factorial_product(4), 24);
    }
}
