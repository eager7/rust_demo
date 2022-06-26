use pct_grep;
use std::{env, process};

fn main() {
    println!("Hello, mgrep!");
    let config = pct_grep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("config err:{}", err);
        process::exit(1);
    });

    if let Err(e) = pct_grep::run(config) {
        eprintln!("app err:{}", e);
        process::exit(1);
    }
}
