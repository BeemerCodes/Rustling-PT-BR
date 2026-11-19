#![allow(clippy::ptr_arg)]

// TODO: Corrija os erros de compilação sem alterar nada, exceto adicionando ou
// removendo referências (o caractere `&`).

// Não deve assumir ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Deve assumir ownership
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}
