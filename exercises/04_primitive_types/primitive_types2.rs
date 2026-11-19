// Caracteres (`char`)

fn main() {
    // Repare nas aspas _simples_: elas são diferentes das aspas duplas
    // que você tem visto por aí.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: De forma análoga ao exemplo anterior, declare uma variável chamada `your_character`
    // abaixo com o seu caractere favorito.
    // Experimente uma letra, um dígito (entre aspas simples), um caractere especial, um caractere
    // de outro idioma que não seja o seu, ou um emoji 😉
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
