fn main() {
    // この行でslice_out_of_array関数のテストができます。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4  <- indices
        //          -------
        //             |
        //             +--- slice

        // 配列のindexが4のものは除外されます。
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // `=`を使うことで配列に記載したindexまで含めることができます。
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
