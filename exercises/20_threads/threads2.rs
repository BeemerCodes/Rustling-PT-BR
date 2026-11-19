// Continuando a partir do exercício anterior, queremos que todas as threads
// concluam seu trabalho. Mas, desta vez, as threads criadas precisam ficar
// responsáveis por atualizar um valor compartilhado: `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` não é suficiente se você quiser um estado compartilhado **mutável**.
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: Você precisa tomar uma providência antes de atualizar um valor compartilhado.
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Aguardando todas as tarefas concluírem.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Imprima o valor de `JobStatus.jobs_done`.
    println!("Jobs done: {}", todo!());
}
