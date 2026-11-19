// A ferramenta Clippy é uma coleção de lints para analisar seu código,
// permitindo detectar erros comuns e melhorar seu código Rust.
//
// Para estes exercícios, o código falhará ao compilar quando houver avisos do
// Clippy. Confira as sugestões do Clippy na saída para resolver o exercício.

fn main() {
    // TODO: Corrija o lint do Clippy nesta linha.
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
