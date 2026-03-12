mod pages;
mod serve;

use crate::serve::ServeManager;
use rand::{RngExt, random};
use std::fmt::format;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let mut manager = ServeManager::new("./pages".into());
    manager.populate_from_root();
    
    
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(&mut stream);

        let mut line = String::new();
        let mut resource = String::new();
        if reader.read_line(&mut line).is_ok() {
            let chunks: Vec<&str> = line.split(" ").collect();
            resource = chunks[1].into();
        };

        let content = manager.serve(resource);
        //let content = format!("<p>Tried to access : {resource}, \nReply : </p><h1>{num}</h1>");
        let len = content.len();
        let headers = format!("Content-Length: {len}\r\nContent-Type: text/html");
        println!("{}", content);
        stream
            .write_all(format!("HTTP/1.1 200 OK\r\n{headers}\r\n\r\n{content}").as_bytes())
            .unwrap();
    }
}
