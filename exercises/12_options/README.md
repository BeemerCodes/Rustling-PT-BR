# Options

O tipo Option representa um valor opcional: cada Option pode ser Some, contendo um valor, ou None, sem valor.
Tipos Option são muito comuns em código Rust, pois têm diversos usos:

- Valores iniciais
- Valores de retorno para funções que não estão definidas em todo o seu intervalo de entrada (funções parciais)
- Valor de retorno para reportar erros simples, quando None é retornado em caso de erro
- Campos opcionais de struct
- Campos de struct que podem ser emprestados ou "retirados"
- Argumentos opcionais de função
- Ponteiros anuláveis
- Trocar valores de lugar para sair de situações complicadas

## Mais informações

- [Formato do Enum Option](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Documentação do módulo Option](https://doc.rust-lang.org/std/option/)
- [Documentação do enum Option](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
