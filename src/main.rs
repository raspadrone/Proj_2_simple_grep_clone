#![allow(dead_code)]
/* This program will:

- Take two command-line arguments: a pattern to search for and a file path.
- Read the contents of the specified file line by line.
- Print only those lines that contain the given pattern.
- Optionally, it should support a case-insensitive search mode, activated by a flag (e.g., -i or --ignore-case).
- Handle errors gracefully (e.g., file not found, permission denied), printing messages to stderr. */

use std::env::args;
#[derive(Debug)]
struct Config {
    query: String,     // the pattern to search for
    file_path: String, // the path to the file to search in
    ignore_case: bool, // true if case-insensitive, false otherwise, default to false
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let args_len = args.len();
        if args_len == 3 {
            let config = Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
                ignore_case: false,
            };
            return Ok(config);
        } else if args_len == 4 {
            let mut config = Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
                ignore_case: false,
            };
            if args.contains(&"-i".to_string()) || args.contains(&"--ignore-case".to_string()) {
                config.ignore_case = true;
            } else {
                return Err("Expected ignore case flag");
            }
            return Ok(config);
        } else if args_len < 3 {
            return Err("Not enough arguments");
        } else {
            return Err("Too many arguments");
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::build(&args);
    match config {
        Ok(config) => println!("{config:?}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
