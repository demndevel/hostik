use std::{env, fs, process};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = match Config::new(&args) { 
        Ok(config) => config,
        _ => {
            eprintln!("Error. Bye");
            process::exit(1);
        }
    };
    
    run(config);
}

struct Config {
    file: String,
    ip: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("")
        }
        
        let file = args[1].clone();
        let ip = args[2].clone();

        Ok(Config { file, ip })
    }
}

fn run(config: Config) {
    let listener = TcpListener::bind(&config.ip).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &config.file);
    }
}

fn handle_connection(mut stream: TcpStream, path: &str) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
