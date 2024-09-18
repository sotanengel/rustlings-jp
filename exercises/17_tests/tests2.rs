// 2の累乗を計算するメソッドです。`1 << n`は`2のn乗`と同義です。
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
  // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: `power_of_2`に値を入れてテストしてみてください。
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}
