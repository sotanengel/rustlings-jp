mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // モジュール外からこのメソッドにアクセスできるように`pub`を追加しました。
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
