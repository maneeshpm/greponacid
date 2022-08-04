use std::{env, process};

use greponacid::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Err parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching [{}] in [{}]", config.query, config.filename);
    println!("-------------------------------------------------------");

    if let Err(e) = greponacid::run(config) {
        println!("Application Err: {}", e);
        process::exit(1);
    }
}

