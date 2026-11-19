# Traits

Uma trait é uma coleção de métodos.

Tipos de dados podem implementar traits. Para isso, os métodos que compõem a trait são definidos para o tipo de dados. Por exemplo, o tipo de dados `String` implementa a trait `From<&str>`. Isso permite que você escreva `String::from("hello")`.

Dessa forma, traits são um pouco parecidas com interfaces em Java e classes abstratas em C++.

Algumas outras traits comuns em Rust incluem:

- `Clone` (o método `clone`)
- `Display` (que permite exibição formatada via `{}`)
- `Debug` (que permite exibição formatada via `{:?}`)

Como traits indicam comportamento compartilhado entre tipos de dados, elas são úteis ao escrever generics.

## Mais informações

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
