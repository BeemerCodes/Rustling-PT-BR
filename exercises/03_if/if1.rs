fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete esta função para retornar o maior número!
    // Se os dois números forem iguais, qualquer um deles pode ser retornado.
    // Não use:
    // - outra chamada de função
    // - variáveis adicionais
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

// Não ligue para isso por enquanto :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
