#![allow(unused_variables)]
use std::net::{TcpListener, TcpStream};
use std::process;
use std::io::prelude::*;
use std::fs;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        println!("connection is failed: {:?}", err);
        process::exit(1);
    });
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    stream.read(&mut buffer).unwrap();
    println!("requests: {}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";
    let status_line_ok = "HTTP/1.1 200 OK\r\n\r\n";
    let status_line_404 = "HTTP/1.1 400 Not Found\r\n\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        (status_line_ok, "./static/hello.html")
    } else {
        (status_line_404, "./static/404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}