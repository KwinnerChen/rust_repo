use std::process;
use std::net::{TcpListener, TcpStream};
use simple_web_server::ThreadPool;
use std::fs;
use std::time::Duration;
use std::thread;
use std::io::prelude::*;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        println!("connection is failed: {:?}", err);
        process::exit(1);
    });

    let pool = ThreadPool::new(5);
    
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.excute(|| {
            handle_connection(stream);
        });
    }

    println!("shutting down!")
}


/// 请求相应接口
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    stream.read(&mut buffer).unwrap();
    println!("requests: {}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let status_line_ok = "HTTP/1.1 200 OK\r\n\r\n";
    let status_line_404 = "HTTP/1.1 400 Not Found\r\n\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        (status_line_ok, "./static/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (status_line_ok, "./static/hello.html")
    } else {
        (status_line_404, "./static/404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}