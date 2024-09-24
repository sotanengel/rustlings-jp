// ある魔法学校ではRustで実装された成績表の生成システムがある。
// 最近までシステムは成績表の評価を数値(e.g. 1.0 -> 5.5)で表現していたが、アルファベット(A+ -> F-)でも成績を印刷できるようにしたい。

// `ReportCard`構造体で必要な変更を行い、impl句で数値での成績に加えて、アルファベットもサポートするように修正してください。

use std::fmt::Display;

// ジェネティクスな型を構造体に導入します。
struct ReportCard<T> {
    //           ^^^
    grade: T,
    //     ^
    student_name: String,
    student_age: u8,
}

// グレードを印刷できるようにするために`Display`トレイトを実装します。
impl<T: Display> ReportCard<T> {
    //  ^^^^^^^ `T`の実装に`Display`が必要です。
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
