# Lifetimes

Os lifetimes dizem ao compilador como verificar se as referências vivem tempo
suficiente para serem válidas em uma determinada situação. Por exemplo, lifetimes
dizem: "garanta que o parâmetro 'a' viva tanto quanto o parâmetro 'b' para que o
valor de retorno seja válido".

Eles só são necessários em borrows, isto é, referências, já que
parâmetros copiados ou movidos têm ownership no próprio escopo e não podem
ser referenciados fora dele. Lifetimes significam que o código que chama
funções, por exemplo, pode ser verificado para garantir que seus argumentos
sejam válidos. Lifetimes impõem restrições a quem faz a chamada.

Se você quiser aprender mais sobre anotações de lifetime, o projeto
[lifetimekata](https://tfpk.github.io/lifetimekata/) tem um estilo de
exercícios parecido com o do Rustlings, mas é totalmente focado em
aprender a escrever anotações de lifetime.

## Mais informações

- [Lifetimes (em Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validando referências com Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
