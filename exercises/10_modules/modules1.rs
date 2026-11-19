// TODO: Corrija o erro de compilação ao chamar uma função privada.
mod sausage_factory {
    // Não deixe ninguém fora deste módulo ver isto!
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
