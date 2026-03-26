mod errors;
mod pages;
mod serve;

use crate::serve::ServeManager;
use regex::Regex;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

/// Entry point: listens at port 8080 and calls the part to populate and serve the pages
fn main() {
    let args = env::args().collect::<Vec<_>>();
    let verbose: bool = args.contains(&String::from("-v"));
    let port = args
        .iter()
        .position(|arg| arg == "-p" || arg == "--port")
        .and_then(|index| args.get(index + 1));
    let mut manager = ServeManager::new("./pages".into(), verbose);
    manager.populate_from_root();

    let port = match port {
        Some(a) => a,
        None => "8080",
    };

    let mut is_valid_request = false;

    // binds the port and iterate over all the clients to serve the pages
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(&mut stream);

        // get the resource asked
        let mut line = String::new();
        let mut resource = String::new();
        if reader.read_line(&mut line).is_ok() {
            if is_valid_get_request(&line) {
                let chunks: Vec<&str> = line.split_whitespace().collect();
                if chunks.len() >= 2 {
                    is_valid_request = true;
                    resource = chunks[1].into();
                }
            } else {
                is_valid_request = false;
            }
        };

        // serve the resource and store the reply to the request
        let headers = "Content-Length: 11\r\nContent-Type: ".to_string();
        let mut reply = format!("HTTP/1.1 400 OK\r\n{headers}\r\n\r\nBad Request");
        if is_valid_request {
            reply = manager.serve(resource);
        }

        stream.write_all(reply.as_bytes()).unwrap();
    }

    pub fn is_valid_get_request(line: &str) -> bool {
        static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        let re = RE.get_or_init(|| {
            Regex::new(
                r"(?x)
            ^GET\s+
            (/[a-zA-Z0-9.\-_~%!$&'()*+,;=:@/]*)     # path
            (?:\?[a-zA-Z0-9.\-_~%!$&'()*+,;=:@/?&]*)? # query
            \s+HTTP/(1\.0|1\.1|2(?:\.0)?)$           # proto
        ",
            )
            .unwrap()
        });

        re.is_match(line.trim())
    }
}
