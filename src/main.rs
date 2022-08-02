use std::{env, process};
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Err parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for: {}\n searching in: {}", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application Err: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone()
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Read contents:\n{}", contents);
    Ok(())
}
