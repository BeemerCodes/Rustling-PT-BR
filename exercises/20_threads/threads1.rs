// Este programa cria múltiplas threads em que cada uma executa por pelo menos 250ms,
// e cada thread retorna quanto tempo levou para concluir. O programa deve
// esperar até que todas as threads criadas tenham terminado e coletar seus
// valores de retorno em um vetor.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Colete os resultados de todas as threads no vetor `results`.
        // Use a struct `JoinHandle` retornada por `thread::spawn`.
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
