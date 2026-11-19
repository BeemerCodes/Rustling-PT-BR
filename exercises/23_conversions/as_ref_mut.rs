// AsRef e AsMut permitem conversões baratas de referência para referência. Leia
// mais sobre eles em https://doc.rust-lang.org/std/convert/trait.AsRef.html e
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectivamente.

// Obtenha o número de bytes (não caracteres) no argumento fornecido
// (`.len()` retorna o número de bytes em uma string).
// TODO: Adicione a trait `AsRef` adequadamente como um trait bound.
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtenha o número de caracteres (não bytes) no argumento fornecido.
// TODO: Adicione a trait `AsRef` adequadamente como um trait bound.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Eleva um número ao quadrado usando `as_mut()`.
// TODO: Adicione o trait bound adequado.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implemente o corpo da função.
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
