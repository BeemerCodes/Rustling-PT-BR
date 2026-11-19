// Este exercício é parecido com o exercício `from_into` anterior. Mas, desta vez,
// vamos implementar `FromStr` e retornar erros em vez de usar um valor padrão.
// Além disso, ao implementar `FromStr`, você pode usar o método `parse` em
// strings para gerar um objeto do tipo implementador. Você pode ler mais sobre
// isso na documentação:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// Usaremos este tipo de erro para a implementação de `FromStr`.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Quantidade incorreta de campos
    BadLen,
    // Campo de nome vazio
    NoName,
    // Erro encapsulado vindo de parse::<u8>()
    ParseInt(ParseIntError),
}

// TODO: Complete esta implementação de `FromStr` para conseguir fazer o parse de uma `Person`
// a partir de uma string no formato "Mark,20".
// Note que você precisará converter o componente da idade para `u8` com algo
// como `"4".parse::<u8>()`.
//
// Passos:
// 1. Divida a string fornecida nas vírgulas presentes nela.
// 2. Se a operação de divisão retornar menos ou mais de 2 elementos, retorne o
//    erro `ParsePersonError::BadLen`.
// 3. Use o primeiro elemento da divisão como o nome.
// 4. Se o nome estiver vazio, retorne o erro `ParsePersonError::NoName`.
// 5. Converta (parse) o segundo elemento da divisão para `u8` como a idade.
// 6. Se o parse da idade falhar, retorne o erro `ParsePersonError::ParseInt`.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
