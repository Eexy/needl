use std::{
    env::{self},
    process,
};

#[derive(Debug)]
struct Config {
    file: String,
    pattern: String,
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let file = match args.next() {
            Some(file) => file,
            None => return Err("no argument passed for file"),
        };

        let pattern = match args.next() {
            Some(pattern) => pattern,
            None => return Err("no argument passed for pattern"),
        };

        Ok(Config { file, pattern })
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(config);
    println!("Hello, world!");
}
