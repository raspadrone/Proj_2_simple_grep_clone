#![allow(dead_code)]
/* This program will:

- Take two command-line arguments: a pattern to search for and a file path.
- Read the contents of the specified file line by line.
- Print only those lines that contain the given pattern.
- Optionally, it should support a case-insensitive search mode, activated by a flag (e.g., -i or --ignore-case).
- Handle errors gracefully (e.g., file not found, permission denied), printing messages to stderr. */

use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Debug)]
struct Config {
    query: String,     // the pattern to search for
    file_path: String, // the path to the file to search in
    ignore_case: bool, // true if case-insensitive, false otherwise, default to false
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut args_iter = args.iter().skip(1); // skip program name

        let query = match args_iter.next() {
            // query - mandatory arg[1]
            Some(arg) => arg.clone(),
            None => return Err("Not enough arguments: query not provided."),
        };

        let file_path = match args_iter.next() {
            // path - mandatory arg[2]
            Some(arg) => arg.clone(),
            None => return Err("Not enough arguments: file_path not provided."),
        };
        let mut ignore_case = false;
        for arg in args_iter {
            if arg == "-i" || arg == "--ignore-case" {
                ignore_case = true;
            } else {
                return Err(
                    "Unrecognized argument or flag. Usage: <query> <file_path> [-i/--ignore-case]",
                );
            }
        }
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&config.file_path)?;
    let reader = BufReader::new(file);

    for (idx, line) in reader.lines().enumerate() {
        match line {
            Ok(line_str) => {
                if config.ignore_case {
                    if line_str
                        .to_lowercase()
                        .contains(&config.query.to_lowercase())
                    {
                        println!("Line {}: {line_str}", idx + 1);
                    }
                } else {
                    if line_str.contains(&config.query) {
                        println!("Line {}: {line_str}", idx + 1);
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::build(&args);
    match config {
        Ok(config) => {
            println!("{config:?}");
            let _ = run(config);
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
