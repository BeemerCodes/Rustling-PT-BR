// Ao realizar operações em elementos de uma coleção, iterators são
// essenciais. Este módulo ajuda você a se familiarizar com a estrutura de uso
// de um iterator e com a forma de percorrer elementos em uma coleção iterável.

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Crie um iterator sobre o array.
        let mut fav_fruits_iterator = todo!();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Substitua `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Substitua `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Substitua `todo!()`
    }
}
