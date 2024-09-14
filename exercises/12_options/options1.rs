// この関数は冷蔵庫のアイスクリームの数を返す関数です。
// 22時より前の時間の場合は5個のアイスが残っており、22時に誰かが全てアイスを食べてしまったため個数は0になってしまいました。
// 23時より遅い時間の場合には`None`を返してください。
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    // TODO: 関数の中身を埋めてください。
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: テストの一部を修正して、オプション型から数値を取得できるようにしてください。
        let icecreams = maybe_icecream(12);

        assert_eq!(icecreams, 5); // ここの値は変えないでください。
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
