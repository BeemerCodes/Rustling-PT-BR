// Usar tipos de erro genéricos ("pega-tudo") como `Box<dyn Error>` não é
// recomendado para código de biblioteca, em que quem chama a função pode querer
// tomar decisões com base no conteúdo do erro em vez de apenas imprimi-lo ou
// propagá-lo. Aqui, definimos um tipo de erro personalizado para permitir que
// quem chama decida o que fazer em seguida quando nossa função retorna um erro.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Um tipo de erro personalizado que vamos usar em `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: Adicione outra função de conversão de erro aqui.
    // fn from_parse_int(???) -> Self { ??? }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // TODO: altere isto para retornar um erro apropriado em vez de dar panic
        // quando `parse()` retornar um erro.
        let x: i64 = s.parse().unwrap();
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
