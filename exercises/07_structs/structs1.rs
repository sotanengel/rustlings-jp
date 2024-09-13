struct ColorRegularStruct {
    // TODO: `regular_structs`テストが想定するように構造体のフィールドを定義してください。
    // 構造体に適切な型はなんでしょうか？
}

struct ColorTupleStruct(/* TODO:`tuple_structs`テストが想定するように構造体のフィールドを定義してください。 */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: 基本的な構造体のインスタンスを作成してください。
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: タプル構造体のインスタンスを作成してください。
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: ユニット構造体のインスタンスを作成してください。
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
