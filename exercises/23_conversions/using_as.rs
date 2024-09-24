// Rustでの型の変換は`as`を利用することで実行される。
// `as`は型変換の時だけでなく、import時の名前の変換にも用いられる。

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: 除算の前に変換を行ってください。
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
