// Estamos coletando frutas diferentes para assar um delicioso bolo de frutas.
// Para isso, temos uma cesta, que vamos representar na forma de um hash map.
// A chave representa o nome de cada fruta que coletamos e o valor representa
// quantas daquela fruta já coletamos. Três tipos de frutas — Apple (4),
// Mango (2) e Lychee (5) — já estão no hash map da cesta. Você precisa
// adicionar frutas à cesta para que haja pelo menos uma de cada tipo e
// mais de 11 no total — temos muitas bocas para alimentar. Você não pode
// inserir mais nenhuma das frutas que já estão na cesta (Apple,
// Mango e Lychee).

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: Insira novas frutas se elas ainda não estiverem presentes na
        // cesta. Note que você não pode colocar nenhum tipo de fruta que já
        // esteja presente!
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Não modifique esta função!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Fruit kind {fruit_kind:?} was not found in basket");
            };
            assert!(*amount > 0);
        }
    }
}
