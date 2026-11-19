# Conversões de tipo

Rust oferece várias maneiras de converter um valor de um determinado tipo para outro tipo.

A forma mais simples de conversão de tipo é uma expressão de *type cast*, indicada pelo operador binário `as`. Por exemplo, `println!("{}", 1 + 1.0);` não compilaria, pois `1` é um inteiro enquanto `1.0` é um float. Porém, `println!("{}", 1 as f32 + 1.0)` compilaria normalmente. O exercício [`using_as`](using_as.rs) cobre esse assunto.

Rust também oferece traits que facilitam conversões de tipos quando implementadas. Essas traits podem ser encontradas no módulo [`convert`](https://doc.rust-lang.org/std/convert/index.html).
Essas traits são as seguintes:

- `From` e `Into`, abordados em [`from_into`](from_into.rs)
- `TryFrom` e `TryInto`, abordados em [`try_from_into`](try_from_into.rs)
- `AsRef` e `AsMut`, abordados em [`as_ref_mut`](as_ref_mut.rs)

Além disso, o módulo `std::str` oferece uma trait chamada [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html), que ajuda a converter strings para tipos de destino por meio do método `parse` em strings. Se estiver implementada corretamente para um tipo `Person`, então `let p: Person = "Mark,20".parse().unwrap()` deve compilar e executar sem entrar em panic.

Essas devem ser as principais formas ***dentro da biblioteca padrão*** de converter dados para os tipos desejados.

## Mais informações

Esses tópicos não são cobertos diretamente no livro, mas a biblioteca padrão tem uma ótima documentação sobre eles.

- [conversões](https://doc.rust-lang.org/std/convert/index.html)
- [trait `FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
