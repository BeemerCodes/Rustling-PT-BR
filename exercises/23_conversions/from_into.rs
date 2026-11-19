// A trait `From` é usada para conversões de valor para valor. Se `From` for
// implementada, uma implementação de `Into` é fornecida automaticamente.
// Você pode ler mais sobre isso na documentação:
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Implementamos a trait Default para usá-la como fallback quando a string
// fornecida não puder ser convertida em um objeto `Person`.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: Complete esta implementação de `From` para conseguir fazer o parse de uma `Person`
// a partir de uma string no formato "Mark,20".
// Note que você precisará converter o componente da idade para `u8` com algo
// como `"4".parse::<u8>()`.
//
// Passos:
// 1. Divida a string fornecida nas vírgulas presentes nela.
// 2. Se a operação de divisão retornar menos ou mais de 2 elementos, retorne o
//    valor padrão (default) de `Person`.
// 3. Use o primeiro elemento da divisão como o nome.
// 4. Se o nome estiver vazio, retorne o valor padrão (default) de `Person`.
// 5. Converta (parse) o segundo elemento da divisão para `u8` como a idade.
// 6. Se o parse da idade falhar, retorne o valor padrão (default) de `Person`.
impl From<&str> for Person {
    fn from(s: &str) -> Self {}
}

fn main() {
    // Use a função `from`.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // Como `From` está implementada para Person, podemos usar `Into`.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
