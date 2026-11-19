// Chamadas a esta função devem ser substituídas por chamadas a `string_slice` ou `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Aqui estão vários valores — alguns são `String`, outros são `&str`.
// Sua tarefa é substituir `placeholder(…)` por `string_slice(…)`
// ou `string(…)`, dependendo do que você acha que cada valor é.
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

    // AVISO: Isto é indexação por byte, não indexação por caractere.
    // A indexação por caractere pode ser feita usando `s.chars().nth(INDEX)`.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
