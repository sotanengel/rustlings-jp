// TODO: この関数は空のString型を入力すると名札の印刷するテキストの生成を拒否する関数です。
// ここで問題が生じた際に単純に`None`を返す代わりに、その問題について説明する方が好ましいです。
// 幸いなことに、Rustにはエラーの状態について説明できるOption型に似た構造体があります。
// 関数の返り値を`Option`型から`Result`型に変更するしてください。
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Emptyに命名をすることはできません。
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
