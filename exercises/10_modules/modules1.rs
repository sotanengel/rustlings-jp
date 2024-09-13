// TODO: プライベートな関数を呼び出しているコンパイルエラーを修正してください。
mod sausage_factory {
    // このメソッドはモジュール外部から呼び出せないように注意してください。
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
