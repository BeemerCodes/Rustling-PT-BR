# Vectors

Vectors são uma das estruturas de dados mais usadas em Rust. Em outras linguagens de
programação, eles seriam chamados simplesmente de Arrays, mas, como Rust opera em um
nível um pouco mais baixo, um array em Rust é armazenado na stack (o que significa que
ele não pode crescer nem diminuir, e o tamanho precisa ser conhecido em tempo de compilação),
já um Vector é armazenado na heap (onde essas restrições não se aplicam).

Vectors aparecem em um capítulo um pouco mais adiante no livro, mas achamos que eles
são úteis o bastante para falarmos deles um pouco mais cedo. Vamos falar sobre a outra
estrutura de dados útil, hash maps, mais tarde.

## Mais informações

- [Armazenando listas de valores com Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
