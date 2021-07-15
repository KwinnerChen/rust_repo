//!使用tokio v1.8构建的简单redis协议服务器。


use mini_redis::{Command::{self, Get, Set}, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use bytes::Bytes;


type StorageType = Arc<Mutex<HashMap<String, Bytes>>>;

lazy_static! {
    static ref RUDB: StorageType = Arc::new(Mutex::new(HashMap::new()));
}


async fn process(stream: TcpStream, db: StorageType) {
    let mut connection = Connection::new(stream);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("Got command: {:?}", cmd);
                let _ = db
                .lock()
                .await
                .insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }

            Get(cmd) => {
                println!("Got command: {:?}", cmd);
                let db = db.lock().await;
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}


#[tokio::main]
async fn main() {
    let addr = env::args()
    .skip(1)
    .next()
    .unwrap_or("localhost:6379".to_owned());

    let linstener = TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", &addr);
    
    loop {
        let (stream, _) = linstener.accept().await.unwrap();
        println!("accept: {:?}", stream);
        // let db = RUDB.clone();
        tokio::spawn(async move {
            process(stream, RUDB.clone()).await;
        });
    }
}
