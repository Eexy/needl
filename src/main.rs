use std::collections::HashMap;
use std::{env, fs, process};

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
/// If unable to read the file then stop process and return error code
fn search(config: &Config) -> HashMap<usize, String> {
    let content = match fs::read_to_string(&config.file) {
        Ok(file_content) => file_content,
        Err(_) => {
            eprintln!("unable to read file");
            process::exit(1)
        }
    };
    let lines: Vec<_> = content.lines().map(String::from).collect();
    let mut result = HashMap::new();
    lines
        .into_iter()
        .enumerate()
        .filter(|(idx, line)| line.contains(&config.pattern))
        .for_each(|(idx, line)| { result.insert(idx, line); });

    result
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let result = search(&config);

    for (idx, line) in result.into_iter() {
        println!("{}: {}", idx, line);
    }
}
