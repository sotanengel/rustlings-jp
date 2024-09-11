fn main() {
    // この行でslice_out_of_array関数のテストができます。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: 以下のテストが通るような`nice_slice`というスライスを配列aから作成してください。
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
