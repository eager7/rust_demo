use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        args.next();
        let query = match args.next() {
            Some(v) => v,
            None => return Err("query invalid"),
        };

        let filename = match args.next() {
            Some(v) => v,
            None => return Err("file invalid"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("read file:{}", config.filename);
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
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
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
