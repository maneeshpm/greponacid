use std::fs;
use std::error::Error;
use std::vec;

// Stuct to hold input
pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    // Create a new config storing user input
    pub fn new(args: &[String]) -> Result<Config, &'static str> { if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone()
        })
    }
}

// Actually call the search
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search_in_content(&config.query, &contents) {
        println!("-> {}", line);
    }
    Ok(())
}

// Perform search on some content
fn search_in_content<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_content() {
        let query = "igg";
        let contents = "first line\nneed bigger tests\nthird line";
        assert_eq!(vec!["need bigger tests"], search_in_content(&query, &contents))
    }
}
