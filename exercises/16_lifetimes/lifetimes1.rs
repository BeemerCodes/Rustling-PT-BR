// O compilador Rust precisa saber como verificar se as referências fornecidas
// são válidas, para poder avisar se uma referência corre o risco de sair de
// escopo antes de ser usada. Lembre-se: referências são borrows e não são
// donas dos próprios dados. E se o dono sair de escopo?

// TODO: Corrija o erro de compilação atualizando a assinatura da função.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
