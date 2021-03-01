//! 因为版本迭代，tokio的API已经更改，按照书中的示例以无法正确运行
//! 稍后再来修改



use futures::Stream;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio_codec::Decoder;
use std::env;


mod codec;
use codec::RespCodec;


mod commands;
use commands::process_client_request;


// 声明一个内存存储库具有静态地址和可变长度
lazy_static! {
    static ref RUDIS_DB: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
    .skip(1)
    .next()
    .unwrap_or("127.0.0.1:6378".to_owned());

    let addr = addr.parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr)?;
    println!("rudis_async listening on: {}", &addr);

    let server = listener
    .incoming()
    .map_err(|e| println!("failed to accept socket; error = {:?}", e))
    .for_each(handle_client);

    tokio::run(server);
    Ok(())
}


fn handle_client(stream: TcpStream) -> Result<(), ()> {
    // unimplemented!()
    let (tx, rx) = RespCodec
    .framed(stream)
    .split();
    let reply = rx
    .and_then(process_client_request);
    let task = tx
    .send_all(reply)
    .then(|res| {
        if let Err(e) = res {
            eprintln!("failed to process connection; error = {:?}", e);
        }
        Ok(())
    });

    tokio::spawn(task);
    Ok(())
}