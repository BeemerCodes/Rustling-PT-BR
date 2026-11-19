// Digamos que estamos escrevendo um jogo em que você pode comprar itens com tokens.
// Todos os itens custam 5 tokens e, sempre que você compra itens, há uma taxa de
// processamento de 1 token. Quem está jogando vai digitar quantos itens quer
// comprar, e a função `total_cost` vai calcular o custo total dos itens. Como a
// pessoa digitou a quantidade, nós a recebemos como uma string. Ela pode ter
// digitado qualquer coisa, não só números!
//
// No momento, esta função não está tratando o caso de erro de jeito nenhum.
// O que queremos fazer é: se chamarmos a função `total_cost` com uma string que
// não é um número, ela vai retornar um `ParseIntError`. Nesse caso, queremos
// retornar esse erro imediatamente da nossa função e não tentar multiplicar
// nem somar.
//
// Existem pelo menos duas maneiras corretas de implementar isso. Mas uma
// delas é bem mais curta!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Trate o caso de erro conforme descrito acima.
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
