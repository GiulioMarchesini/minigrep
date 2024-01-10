use std::error::Error; // per gestire gli errori. è come use namespace in c++

/// Struct che contiene i dati passati da terminale
pub struct Config {
    pub query: String,     // la stringa da cercare
    pub file_path: String, // il percorso del file da leggere
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // se non ci sono abbastanza argomenti
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // dyn indica che il tipo di errore è dinamico, può essere di qualsiasi tipo
    let contents = std::fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);

    Ok(()) // non ritorna niente, serve solo per indicare che la funzione è andata a buon fine
}
