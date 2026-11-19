// É preciso definir uma cesta de frutas na forma de um hash map. A chave
// representa o nome da fruta e o valor representa quantas unidades daquela
// fruta há na cesta. Você precisa colocar pelo menos 3 tipos diferentes
// de frutas (ex.: maçã, banana, manga) na cesta, e a contagem total
// de todas as frutas deve ser de pelo menos 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare o hash map.
    // let mut basket =

    // Duas bananas já vêm de brinde para você :)
    basket.insert(String::from("banana"), 2);

    // TODO: Coloque mais frutas na sua cesta.

    basket
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
