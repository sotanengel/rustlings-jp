fn main() {
    // この行で関数のテストができます。 
}

#[cfg(test)]
mod tests {
    // TODO: テスト内の行を入れ替えることでコンパイルエラーを修正してください。
    // その他の行を追加したり削除したりしないでください。
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        let z = &mut x;
        y.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
