// Calcula a potência de 2 usando bit shift (deslocamento de bits).
// `1 << n` equivale a "2 elevado à potência de n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Teste a função `power_of_2` com alguns valores.
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}
