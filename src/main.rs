// Custom library defined for the project
use minigrep::run;
use minigrep::Config; // Shortcut to use the standard library, similar to using namespace in C++
use std::env; // To read arguments passed from the terminal

fn main() {
    // env::args() returns an iterator containing the arguments passed to the program
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // If the build function returns an error, print the message and terminate the program
        eprintln!("Problem parsing arguments: {}", err); // eprintln prints to stderr instead of stdout
        std::process::exit(1); // 1 indicates that the program terminated abnormally
    });

    // Read the file
    if let Err(e) = run(config) {
        // If the run function returns an error, print the message and terminate the program
        eprintln!("Application error: {e}");
        std::process::exit(1); // 1 indicates that the program terminated abnormally
    }
}
