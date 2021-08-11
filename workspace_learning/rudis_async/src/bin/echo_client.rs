//! 用于测试回显服务器的客户端模块

use std::env;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, task::JoinHandle};


#[tokio::main]
async fn main() {
    let addr = env::args()
    .skip(1)
    .next()
    .unwrap_or("127.0.0.1:8080".to_string());

    let mut tasks: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..500 {
        let addr = addr.clone();
        let message = format!("hello from num {}", i);
        let handle = tokio::spawn(async move {
            client(addr, message).await
        });

        tasks.push(handle);
    }

    for handle in tasks {
        match handle.await {
            Err(e) => eprintln!("wrong: {}", &e),
            Ok(_) => continue,
        }
    }
}


async fn client(addr: String, message: String) {
    let mut stream = TcpStream::connect(&addr).await.unwrap();
    // 必须制定缓存的长度，否则无法写入
    // 如Vec::new()无法用作缓存，但可以存储读取到缓存中的信息(vec![0;1024]可用作缓存)
    let mut buf = [0;1024];

    stream.write_all(&message.as_bytes()).await.unwrap();
    println!("send message: {} {}", &addr, &message);

    // 正常读取应该是一个循环
    // 直到读取返回为Ok(0)或者错误
    let data = stream.read(&mut buf).await.unwrap();
    println!("recieve message: {} {} {}", &stream.peer_addr().unwrap(), String::from_utf8_lossy(&buf), data);
}