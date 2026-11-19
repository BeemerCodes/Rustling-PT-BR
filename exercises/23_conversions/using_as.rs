// Type casting em Rust é feito por meio do uso do operador `as`.
// Note que o operador `as` não é usado apenas para type casting. Ele também
// ajuda a renomear imports.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Faça uma conversão antes de dividir.
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
