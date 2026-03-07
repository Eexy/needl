use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, process};

#[derive(Debug)]
struct Config {
    file: String,
    pattern: String,
}

impl Config {
    fn new(mut args: impl Iterator<Item=String>) -> Result<Self, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(pattern) => pattern,
            None => return Err("no argument passed for pattern"),
        };

        let file = match args.next() {
            Some(file) => file,
            None => return Err("no argument passed for file"),
        };

        Ok(Config { file, pattern })
    }
}

/// Search file content for matching line
/// If unable to read the file then  return error
fn search(config: &Config) -> Result<HashMap<usize, String>, String> {
    let file = match File::open(&config.file) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let reader = BufReader::new(file);
    let mut result = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                if l.contains(&config.pattern) {
                    result.insert(idx, l);
                }
            }
            Err(e) => return Err(e.to_string())
        }
    }


    Ok(result)
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let result = search(&config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    for (idx, line) in result.into_iter() {
        println!("{}: {}", idx, line);
    }
}
