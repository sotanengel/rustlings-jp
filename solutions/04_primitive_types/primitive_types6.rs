fn main() {
    // この行でindexing_tuple関数のテストができます。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // タプルのインデックスを使ったアクセスは以下のように書きます。
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
