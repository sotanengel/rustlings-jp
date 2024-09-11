fn main() {
    // この行でindexing_tuple関数のテストができます。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: `numbers`タプルの二つ目の要素にインデックスを使ってアクセスし、`second`変数に代入しましょう。
        // let second = ???;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
