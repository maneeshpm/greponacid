use std::fs;
use std::error::Error;
use std::vec;

// Stuct to hold input
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool
}

impl Config {
    // Create a new config storing user input
    pub fn new(args: &[String]) -> Result<Config, &'static str> { 
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_insensitive: Config::case_insensitive(args)
        })
    }

    fn case_insensitive(args: &[String]) -> bool {
        if args.len() < 4 {
            return false;
        }

        match args[3].as_str() {
            "T" | "t" | "true" | "True" => true,
            _ => false
        }
    }
}

// Actually call the search
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_insensitive {
        search_in_content_ci(&config.query, &contents)
    } else {
        search_in_content(&config.query, &contents)
    };

    if result.is_empty() {
        println!("No Matches found!");
    }

    for line in result {
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

// Perform cases insenitive search on some content
fn search_in_content_ci<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
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

    #[test]
    fn search_case_insensitive() {
        let query = "IgG";
        let contents = "first line\nneed bigger tests\nthird line";
        assert_eq!(vec!["need bigger tests"], search_in_content_ci(&query, &contents))
    }
}
