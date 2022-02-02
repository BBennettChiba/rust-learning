use std::{error::Error, fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        case_insensitive_search(&config.query, &content)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let case_sensitive = match env::var("CASE_SENSITIVE"){
            Err(_)=> false,
            Ok(flag)=> flag.parse::<bool>().unwrap_or(false)
        };
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let query = "you";
        let contents = "/
Hello there. 
How are you?
I'm fine.";
        assert_eq!(vec!["How are you?"], search(query, contents));
    }
    #[test]
    fn test2() {
        let query = "you";
        let contents = "/
Pizza
is
good";
        assert_eq!(true, search(query, contents).is_empty());
    }
    #[test]
    fn test3() {
        let query = "You";
        let contents = "/
Hello there. 
How are you?
I'm fine.";
        assert_eq!(true, search(query, contents).is_empty());
    }
    #[test]
    fn test4() {
        let query = "rust";
        let contents = "/
Hello there.
How are you?
Rust is cool
Trust me";
        assert_eq!(
            vec!["Rust is cool", "Trust me"],
            case_insensitive_search(query, contents)
        );
    }
}
