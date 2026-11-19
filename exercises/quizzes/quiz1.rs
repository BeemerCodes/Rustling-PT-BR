// Este é um quiz para as seguintes seções:
// - Variáveis
// - Funções
// - If
//
// Mary está comprando maçãs. O preço de uma maçã é calculado assim:
// - Uma maçã custa 2 rustbucks.
// - Porém, se Mary comprar mais de 40 maçãs, o preço de cada maçã no
// pedido inteiro cai para apenas 1 rustbuck!

// TODO: Escreva uma função que calcule o preço de um pedido de maçãs dada
// a quantidade comprada.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn main() {
    // Você pode experimentar aqui, se quiser.
}

// Não altere os testes!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
