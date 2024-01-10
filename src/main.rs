// GREP : Globally search a Regular Expression and print
// $ cargo run -- searchstring example-filename.txt
// esempio di comando per usare il programma:
// $ cargo run -- Are poem.txt

// libreria definita da me per il progetto
use minigrep::run;
use minigrep::Config;
// shortcut per usare la libreria standard, tipo using namespace in c++
use std::env; // per leggere gli argomenti passati dal terminale

fn main() {
    // 1 leggere gli argomenti passati da terminale. il primo elemento è il nome del programma, poi ci sono gli argomenti
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // 2 separare gli argomenti in due variabili
    let config = Config::build(&args).unwrap_or_else(|err| {
        // se la funzione build ritorna un errore, stampa il messaggio e termina il programma
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1); // 1 indica che il programma è terminato in modo anomalo
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // 3 leggere il file
    if let Err(e) = run(config) {
        // se la funzione run ritorna un errore, stampa il messaggio e termina il programma
        println!("Application error: {e}");
        std::process::exit(1); // 1 indica che il programma è terminato in modo anomalo
    }
}
