use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::env;
use std::process;
use blockchain_in_rus::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Program parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = blockchain_in_rus::grep(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    // http_server();
}


fn http_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        // println!("Connection established!\n");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
