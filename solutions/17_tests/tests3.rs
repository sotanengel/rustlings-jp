struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // この関数は変更しないでください。
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // `Result`型を返すことが好ましいかもしれないが、
            // テスト関数がどのようにパニックを起こすか学びたいためこの表記にしている。
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 横幅の確認
        assert_eq!(rect.height, 20); // 高さの確認
    }

    #[test]
    #[should_panic] // テストがパニックを引き起こすかこの属性を加えて確認する。
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic] // テストがパニックを引き起こすかこの属性を加えて確認する。
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
