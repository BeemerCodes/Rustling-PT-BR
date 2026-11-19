// Este poderoso wrapper oferece a capacidade de armazenar um valor inteiro positivo.
// TODO: Reescreva-o usando generics para que ele suporte encapsular QUALQUER tipo.
struct Wrapper {
    value: u32,
}

// TODO: Adapte a implementação da struct para ser genérica sobre o valor encapsulado.
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
