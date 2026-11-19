// Esta loja está em promoção: se o preço for um número par, você ganha 10
// Rustbucks de desconto, mas se for um número ímpar, ganha 3 Rustbucks de desconto.
// Não se preocupe com os corpos das funções por enquanto — estamos
// interessados apenas nas assinaturas.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Corrija a assinatura da função.
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
