use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    if config.case_sensitive {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in search_insensitive(&config.query, &contents) {
            println!("{}", line);
        }
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    return result;
}
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.";

        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "Duct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.";
        assert_eq!(
            vec!["safe,fast,productive."],
            search_insensitive(query, contents)
        );
    }
}
