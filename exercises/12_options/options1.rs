// Esta função retorna quanto sorvete ainda resta na geladeira.
// Se for antes das 22:00 (formato 24 horas), sobram 5 bolas. Às 22:00,
// alguém come tudo, então não sobra sorvete (valor 0). Retorne `None` se
// `hour_of_day` for maior que 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete o corpo da função.
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Corrija este teste. Como você obtém o valor contido no
        // Option?
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // Não altere esta linha.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
