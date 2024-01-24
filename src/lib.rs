use std::env;
use std::error::Error; // per gestire gli errori. è come use namespace in c++ // per leggere le variabili d'ambiente

/// Struct che contiene i dati passati da terminale
pub struct Config {
    pub query: String,     // la stringa da cercare
    pub file_path: String, // il percorso del file da leggere
    pub ignore_case: bool, // se true, la ricerca è case insensitive
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // se non ci sono abbastanza argomenti
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok(); // controlla se la variabile d'ambiente esiste, il valore non è importante

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // dyn indica che il tipo di errore è dinamico, può essere di qualsiasi tipo
    let contents = std::fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(()) // non ritorna niente, serve solo per indicare che la funzione è andata a buon fine
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); // vettore che conterrà i risultati
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // la funzione to_lowercase() restituisce una nuova stringa, quindi non è necessario usare &mut
    let query = query.to_lowercase();
    let mut results = Vec::new(); // vettore che conterrà i risultati
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; // per andare a capo bisogna mettere \ all'inizio della riga

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
