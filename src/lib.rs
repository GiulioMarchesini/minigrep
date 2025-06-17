use std::env;
use std::error::Error; // To handle errors. It's similar to 'use namespace' in C++ // To read environment variables

/// Struct containing data passed from the terminal
pub struct Config {
    pub query: String,     // The string to search for
    pub file_path: String, // The path of the file to read
    pub ignore_case: bool, // If true, the search is case insensitive
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>, // Accepts any iterator that returns a string
    {
        args.next(); // Ignore the first argument, which is the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok(); // Checks if the environment variable exists; the value is not important

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 'dyn' indicates that the error type is dynamic and can be of any type
    let contents = std::fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(()) // Returns nothing, only indicates that the function executed successfully
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
Duct tape."; // To go to the next line, you need to put \ at the beginning of the line

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
