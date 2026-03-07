use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, process};

#[derive(PartialEq, Debug)]
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
/// If error during reading a line stop searching immediately
fn search(pattern: &str, reader: impl BufRead) -> Result<HashMap<usize, String>, String> {
    let mut result = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                if l.contains(pattern) {
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
    let file = match File::open(&config.file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("unable to read file: {err}");
            process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    let result = search(&config.pattern, reader).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    for (idx, line) in result.into_iter() {
        println!("{}: {}", idx, line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_with_valid_args() {
        let args = vec!["app".to_string(), "pattern".to_string(), "file".to_string()];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.pattern, "pattern".to_string());
        assert_eq!(config.file, "file".to_string());
    }

    #[test]
    fn test_config_with_missing_file_args() {
        let args = vec!["app".to_string(), "pattern".to_string()];
        let result = Config::new(args.into_iter());
        assert!(result.is_err());
        assert_eq!(result, Err("no argument passed for file"));
    }

    #[test]
    fn test_config_with_missing_file_pattern() {
        let args = vec!["app".to_string()];
        let result = Config::new(args.into_iter());
        assert!(result.is_err());
        assert_eq!(result, Err("no argument passed for pattern"));
    }


    #[test]
    fn test_search_finds_matching_lines() {
        let content = b"hello world\nrust\nworld";
        let reader = BufReader::new(&content[..]);

        let result = search(&"world", reader).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&0), Some(&"hello world".to_string()));
    }

    #[test]
    fn test_search_with_no_match() {
        let content = b"world\nrust\nworld";
        let reader = BufReader::new(&content[..]);

        let result = search(&"hello", reader).unwrap();
        assert_eq!(result.len(), 0);
    }


    #[test]
    fn test_search_with_case_sensitive() {
        let content = b"world\nrust\nWorld";
        let reader = BufReader::new(&content[..]);

        let result = search(&"World", reader).unwrap();
        assert_eq!(result.len(), 1);
    }
}
