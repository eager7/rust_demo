use std::{env, fs, process};
use std::error::Error;

fn main() {
    println!("Hello, mgrep!");
    let args:Vec<String> = env::args().collect();
    println!("args:{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("config err:{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("app err:{}", e);
        process::exit(1);
    }


}

struct Config {
    query : String,
    filename : String,
}

impl Config {
    fn new(args: &[String])->Result<Config,&'static str> {
        if args.len()<3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config{query, filename});
    }
}

fn run(config :Config)->Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("content:{}", contents);
    return Ok(())
}