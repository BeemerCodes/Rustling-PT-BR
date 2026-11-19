// Em tempo de compilação, o Rust precisa saber quanto espaço um tipo ocupa. Isso
// se torna problemático para tipos recursivos, em que um valor pode conter como
// parte de si mesmo outro valor do mesmo tipo. Para contornar esse problema,
// podemos usar `Box` — um smart pointer usado para armazenar dados na heap,
// que também nos permite envolver um tipo recursivo.
//
// O tipo recursivo que estamos implementando neste exercício é a "cons list",
// uma estrutura de dados comum em linguagens de programação funcional. Cada
// item em uma cons list contém dois elementos: o valor do item atual e o
// próximo item. O último item é um valor chamado `Nil`.

// TODO: Use `Box` na definição do enum para fazer o código compilar.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: Crie uma cons list vazia.
fn create_empty_list() -> List {
    todo!()
}

// TODO: Crie uma cons list não vazia.
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
