use std::{
    env::{self},
    fs, process,
};

#[derive(Debug)]
struct Config {
    file: String,
    pattern: String,
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
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

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let content = match fs::read_to_string(config.file) {
        Ok(file_content) => file_content,
        Err(_) => {
            eprintln!("unable to read file");
            process::exit(1)
        }
    };
    let lines: Vec<_> = content.lines().map(String::from).collect();
    for i in 0..lines.len() {
        if lines[i].contains(&config.pattern) {
            println!("{}:{}", i, lines[i]);
        }
    }
}
