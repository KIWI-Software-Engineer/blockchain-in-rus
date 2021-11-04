use std::env;
use std::process;
use blockchain_in_rus::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Program parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = blockchain_in_rus::grep(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    // http_server();
}


