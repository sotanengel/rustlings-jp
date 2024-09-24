// 正の整数の値を格納する機能を提供する`Wrapper`です。
// 他の型でもラップできるように修正してください。
struct Wrapper {
  value: u32,
}

// TODO: 構造体の実装を、ラップされた値に対してジェネリックになるように適合させる。
impl <T> Wrapper<T>  {
    fn new (value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
