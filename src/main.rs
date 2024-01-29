// GREP : Globally search a Regular Expression and print
// $ cargo run -- searchstring example-filename.txt
// esempio di comando per usare il programma case sensitive:
// $ cargo run -- body poem.txt
// esempio di comando per usare il programma case insensitive:
// $ IGNORE_CASE=1 cargo run -- body poem.txt
// eswempio di comando per stampare il risultato su file e gli errori sul terminale:
// $ cargo run -- to poem.txt > output.txt

// libreria definita da me per il progetto
use minigrep::run;
use minigrep::Config; // shortcut per usare la libreria standard, tipo using namespace in c++
use std::env; // per leggere gli argomenti passati dal terminale

fn main() {
    // 1 leggere gli argomenti passati da terminale. il primo elemento è il nome del programma, poi ci sono gli argomenti
    let args: Vec<String> = env::args().collect();

    // 2 separare gli argomenti in due variabili
    let config = Config::build(&args).unwrap_or_else(|err| {
        // se la funzione build ritorna un errore, stampa il messaggio e termina il programma
        eprintln!("Problem parsing arguments: {}", err); // eprintln stampa su stderr invece che stdout
        std::process::exit(1); // 1 indica che il programma è terminato in modo anomalo
    });

    // 3 leggere il file
    if let Err(e) = run(config) {
        // se la funzione run ritorna un errore, stampa il messaggio e termina il programma
        eprintln!("Application error: {e}");
        std::process::exit(1); // 1 indica che il programma è terminato in modo anomalo
    }
}
