// Este programa está tentando usar uma versão completa da função
// `total_cost` do exercício anterior. Porém, ele não está funcionando!
// Por que não? O que devemos fazer para corrigir isso?

use std::num::ParseIntError;

// Não altere esta função.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Corrija o erro de compilação alterando a assinatura e o corpo da
// função `main`.
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Não altere esta linha.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
}
