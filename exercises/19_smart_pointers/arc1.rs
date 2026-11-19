// Neste exercício, recebemos um `Vec` de `u32` chamado `numbers` com valores
// variando de 0 a 99. Queremos usar esse conjunto de números em 8 threads
// diferentes simultaneamente. Cada thread vai calcular a soma de um a cada
// oito valores, a partir de um deslocamento (offset).
//
// A primeira thread (offset 0) vai somar 0, 8, 16, …
// A segunda thread (offset 1) vai somar 1, 9, 17, …
// A terceira thread (offset 2) vai somar 2, 10, 18, …
// …
// A oitava thread (offset 7) vai somar 7, 15, 23, …
//
// Cada thread deve possuir um ponteiro com contagem de referências para o
// vetor de números. Porém, `Rc` não é thread-safe. Por isso, precisamos usar `Arc`.
//
// Não se distraia com a forma como as threads são criadas e sincronizadas.
// Vamos praticar isso mais tarde nos exercícios sobre threads.

// Não altere as linhas abaixo.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Defina `shared_numbers` usando `Arc`.
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Defina `child_numbers` usando `shared_numbers`.
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
