# Minigrep

Minigrep is a simple implementation of the grep (GREP: Globally search a Regular Expression and Print) command-line tool written in Rust. It allows you to search for a specific string in a file and displays the lines containing the string.

## Features

- Search for a string in a file.

- Case-sensitive and case-insensitive search options.

## Usage

Run the program with the following command:

```bash
minigrep <query> <filename>
```

- `<query>`: The string to search for.

- `<filename>`: The file to search in.

## Example

To search for the word "example" in a file named `poem.txt`, use:

```bash
minigrep example poem.txt
```

## Build

To build the project, use Cargo:

```bash
cargo build
```

## Run

To run the project, use Cargo:

```bash
cargo run <query> <filename>
```

## License

This project is licensed under the MIT License.
