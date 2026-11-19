// `Vec<T>` é genérico sobre o tipo `T`. Na maioria dos casos, o compilador consegue
// inferir `T`, por exemplo após inserir um valor com tipo concreto no vetor.
// Mas, neste exercício, o compilador precisa de uma ajudinha por meio de uma anotação de tipo.

fn main() {
    // TODO: Corrija o erro de compilação anotando o tipo do vetor
    // `Vec<T>`. Escolha `T` como um tipo inteiro que possa ser criado a partir de
    // `u8` e `i8`.
    let mut numbers = Vec::new();

    // Não altere as linhas abaixo.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
