// これは以下のセクションのクイズです：
// - Generics
// - Traits
//
// ある魔法学校ではRustで実装された成績表の生成システムがある。
// 最近までシステムは成績表の評価を数値(e.g. 1.0 -> 5.5)で表現していたが、アルファベット(A+ -> F-)でも成績を印刷できるようにしたい。

// `ReportCard`構造体で必要な変更を行い、impl句で数値での成績に加えて、アルファベットもサポートするように修正してください。

// TODO: 仕様に合うように構造体を変更してください。
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: 仕様に合うようにimpl内を変更してください。
impl ReportCard {
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
