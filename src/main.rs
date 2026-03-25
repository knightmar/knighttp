mod errors;
mod pages;
mod serve;

use crate::serve::ServeManager;
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

    // binds the port and iterate over all the clients to serve the pages
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(&mut stream);

        // get the resource asked
        let mut line = String::new();
        let mut resource = String::new();
        if reader.read_line(&mut line).is_ok() {
            let chunks: Vec<&str> = line.split(" ").collect();
            resource = chunks[1].into();
        };

        // serve the resource and store the reply to the request
        let reply = manager.serve(resource);

        stream.write_all(reply.as_bytes()).unwrap();
    }
}
