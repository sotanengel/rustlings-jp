// メアリーはりんごを買っています。りんごの価格は以下のように計算されます：
// - りんご1個は2 rustbucksかかります
// - 40個より多くリンゴを買ったら、りんご1個の値段は1 rustbucksになります。

fn calculate_price_of_apples(n_apples: u64) -> u64 {
    if n_apples > 40 {
        n_apples
    } else {
        2 * n_apples
    }
}

fn main() {
    // この行で関数のテストができます。
}

// このテストは変更しないでください。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
