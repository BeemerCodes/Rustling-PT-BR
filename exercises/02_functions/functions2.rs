// TODO: Adicione o tipo que está faltando para o argumento `num` após os dois-pontos `:`.
fn call_me(num:) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
