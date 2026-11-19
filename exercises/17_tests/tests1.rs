// Testes são importantes para garantir que seu código faz o que você acha
// que ele deveria fazer.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    // TODO: Importe `is_even`. Você pode usar um wildcard para importar tudo
    // do módulo externo.

    #[test]
    fn you_can_assert() {
        // TODO: Teste a função `is_even` com alguns valores.
        assert!();
        assert!();
    }
}
