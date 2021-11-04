use std::fs;
use std::error::Error;
use std::env;
pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args.get(1).unwrap().clone();
        let filename = args.get(2).unwrap().clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn grep(config: Config) -> Result<(), Box<dyn Error>>  {

    let contents = fs::read_to_string(&config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
        // .expect("Something went wrong reading the file");

    // println!("Searching for {:?} in file {:?}", config.query, &config.filename);
    // println!("file content is \n\r{}", contents);
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
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
    fn case_sensitive() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, prod";
        assert_eq!(vec!["safe, fast, prod"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let query1 = "rUsTd";
        let contents = "\
Rust:
safe, fast, prod.
Trust me.";
        // let query1 = "rUsTd";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
        // assert_eq!(Vec<&str>, search_case_insensitive(query1, contents));
    }
}