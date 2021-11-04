use std::fs;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args.get(1).unwrap().clone();
        let filename = args.get(2).unwrap().clone();
        Ok(Config {query, filename})
    }
}

pub fn grep(config: Config) -> Result<(), Box<dyn Error>>  {

    let contents = fs::read_to_string(&config.filename)?;
        // .expect("Something went wrong reading the file");

    println!("Searching for {:?} in file {:?}", config.query, &config.filename);
    println!("file content is \n\r{}", contents);
    Ok(())
}
