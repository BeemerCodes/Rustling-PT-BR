struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Não altere esta função.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Retornar um `Result` seria melhor aqui. Mas queremos aprender
            // como testar funções que podem dar panic.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: Este teste deve verificar se o retângulo tem o tamanho que
        // passamos para o construtor dele.
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // Verifica a largura
        assert_eq!(todo!(), 20); // Verifica a altura
    }

    // TODO: Este teste deve verificar se o programa dá panic quando tentamos
    // criar um retângulo com largura negativa.
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: Este teste deve verificar se o programa dá panic quando tentamos
    // criar um retângulo com altura negativa.
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
