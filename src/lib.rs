use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive})
    }    
}

// trait object Box<dyn Error>
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

// Search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new(); // Compilers infers type

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let q = query.to_lowercase();
    let mut results = Vec::new(); // Compilers infers type

    for line in contents.lines() {
        if line.to_lowercase().contains(&q) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Not enough arguments")]
    fn config_without_parameters() {
        let args: Vec<String> = Vec::new();

        if let Err(e) = Config::new(&args) {
            panic!("{}",e);
        }
    }

    #[test]
    #[should_panic(expected = "Not enough arguments")]
    fn config_not_enough_parameters() {
        let mut args: Vec<String> = Vec::new();
        args.push("binary_name".to_string());
        args.push("search_terms".to_string());

        if let Err(e) = Config::new(&args) {
            panic!("{}",e);
        }
    }

    #[test]
    fn config_with_parameters() {
        let mut args: Vec<String> = Vec::new();
        args.push("binary_name".to_string());
        args.push("search_terms".to_string());
        args.push("file_name".to_string());

        let config_created = match Config::new(&args) {
            Ok(_) => true,
            Err(_) => false
        };

        assert!(config_created);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
