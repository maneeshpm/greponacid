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
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> { 
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("No query found")
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("No filename found")
        };

        let case_insensitive = match args.next() {
            Some(c) => match c.as_str() {
                "true" | "True" | "t" | "T" => true,
                _ => false
            } ,
            None => false
        };

        Ok(Config {
            query,
            filename,
            case_insensitive
        })
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
    content.lines().filter(|line| line.contains(query)).collect()
}

// Perform cases insenitive search on some content
fn search_in_content_ci<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
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
    fn search_multiple() {
        let query = "igg";
        let contents = "first line\nneed bigger tests\nthird line\nigg with me";
        assert_eq!(vec!["need bigger tests", "igg with me"], search_in_content(&query, &contents))
    }

    #[test]
    fn search_none() {
        let query = "igg";
        let contents = "first line\nneed smaller tests\nthird line";
        assert!(search_in_content(&query, &contents).is_empty())
    }

    #[test]
    fn search_case_insensitive() {
        let query = "IgG";
        let contents = "first line\nneed bigger tests\nthird line";
        assert_eq!(vec!["need bigger tests"], search_in_content_ci(&query, &contents))
    }
}
