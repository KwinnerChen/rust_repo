//!创建一个实现简单get和set命令的rust版本的redis服务端
//!redis的RESP应用协议由开源库resp解析
#![allow(unused_imports)]

use lazy_static::lazy_static;
use resp::Decoder;
use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;

mod commands;
use commands::process_client_request;

// 内存库中存储类型别名
type STORE = Mutex<HashMap<String, String>>;

// 声明一个动态的静态变量用于存储数据
lazy_static! {
    static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}


fn main() {
    let addr = env::args()
    .skip(1)
    .next()
    .unwrap_or("127.0.0.1:6378".to_string());

    // TODO:这里可以使用更好的错误处理
    let listener = TcpListener::bind(&addr).unwrap();
    println!("rudis_sync listening on {}...", &addr);

    for stream in listener.incoming() {
        // TODO:这里可以使用更好的错误处理
        let stream = stream.unwrap();
        println!("New connection from: {:?}", &stream);

        // 用于处理一个链接请求
        // 这里可以使用线程池，Rudis_db是Mutex
        handle_client(stream);
    }
}

fn handle_client(stream: TcpStream) {
    // 构建一个缓冲结构，不用手动管理缓冲区，具有统一的读写方法
    let mut stream = BufReader::new(stream);
    // 构建解码器，并从中解码出合适的值的Result枚举
    let decoder = Decoder::new(&mut stream).decode();
    match decoder {
        Ok(v) => {
            let reply = process_client_request(v);
            stream.get_mut().write_all(&reply).unwrap();
        }
        Err(e) => {
            println!("Invalid command: {:?}", e);
            let _ = stream.get_mut().shutdown(Shutdown::Both);
        }
    }
}