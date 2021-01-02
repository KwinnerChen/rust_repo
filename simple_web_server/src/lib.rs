use std::net::{TcpStream};
use std::thread;
use std::time::Duration;
use std::fs;
use std::io::prelude::*;


const GET: &[u8] = b"GET / HTTP/1.1\r\n";
const SLEEP: &[u8] = b"GET /sleep HTTP/1.1\r\n";
const STATUS_LINE_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
const STATUS_LINE_404: &str = "HTTP/1.1 400 Not Found\r\n\r\n";


/// 请求相应接口
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    // read返回读取字节的Result
    // 当缓冲区非零而返回值为零时，则流数据读取到尽头
    // read是一个阻塞的方法
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    let (status_line, filename) = if buffer.starts_with(GET) {
        (STATUS_LINE_OK, "./static/hello.html")
    } else if buffer.starts_with(SLEEP) {
        thread::sleep(Duration::from_secs(5));
        (STATUS_LINE_OK, "./static/hello.html")
    } else {
        (STATUS_LINE_404, "./static/404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}