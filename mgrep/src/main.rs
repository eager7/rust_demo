use std::{env, process};
use mgrep;

fn main() {
    println!("Hello, mgrep!");
    let args: Vec<String> = env::args().collect();
    println!("args:{:?}", args);
    let config = mgrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("config err:{}", err);
        process::exit(1);
    });

    if let Err(e) = mgrep::run(config) {
        eprintln!("app err:{}", e);
        process::exit(1);
    }
}
