// Este exercício explora o smart pointer `Cow` (Clone-On-Write). Ele pode
// encapsular e fornecer acesso imutável a dados emprestados (borrowed) e clonar
// os dados de forma preguiçosa (lazy) quando mutação ou ownership é necessário.
// O tipo foi projetado para funcionar com dados emprestados genéricos via trait `Borrow`.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clona para um vetor se ainda não tiver ownership (owned).
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Ocorre um clone porque `input` precisa ser mutado.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // Não ocorre clone porque `input` não precisa ser mutado.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // Também podemos passar `vec` sem `&` para que `Cow` tenha ownership direto.
        // Nesse caso, nenhuma mutação ocorre (todos os números já são
        // absolutos) e, portanto, nenhum clone também. Mas o resultado continua
        // sendo owned porque nunca foi emprestado (borrowed) nem mutado.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // Claro que isso também vale se ocorrer uma mutação (nem todos os
        // números são absolutos). Nesse caso, a chamada a `to_mut()` na
        // função `abs_all` retorna uma referência aos mesmos dados de antes.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }
}
