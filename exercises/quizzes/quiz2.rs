// Este é um quiz para as seguintes seções:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Vamos construir uma maquininha em forma de função. Como entrada, vamos
// passar uma lista de strings e comandos. Esses comandos determinam qual ação
// será aplicada à string. Pode ser:
// - Colocar a string em maiúsculas (Uppercase)
// - Remover espaços das extremidades da string (Trim)
// - Anexar "bar" ao final da string uma quantidade especificada de vezes
//
// O formato exato será:
// - A entrada será um Vector de tuplas de 2 elementos,
//   o primeiro elemento é a string e o segundo é o comando.
// - O elemento de saída será um vetor de strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete a função conforme descrito acima.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    // TODO: O que precisamos importar para ter `transformer` no escopo?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
