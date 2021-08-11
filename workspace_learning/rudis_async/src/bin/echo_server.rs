use std::{env, time::Duration};
use rand::Rng;
use tokio::{net::{TcpListener, TcpStream}, time::sleep};


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let addr = env::args()
    .skip(1)
    .next()
    .unwrap_or("127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(addr).await?;
    println!("servering on {}", &listener.local_addr()?.to_string());

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("{} connected", addr.to_string());

        tokio::spawn(async move {
            handle(stream).await
        });
    }
}


async fn handle(mut stream: TcpStream) {
    let (mut rx, mut wx) = stream.split();

    // tokio::spwan需要类型具有Send特征，因为tokio默认的是一个多线程的运行时
    // 值可能在线程件传递
    // 基于rust严格的作用域和生命周期规则
    // 增加一个代码块，用来将不具有Send特征的类型在传递前调用Drop特征删除
    let secs = {
        let mut rng = rand::thread_rng();
        rng.gen_range(2..=10)
    };

    sleep(Duration::from_secs(secs)).await;
    if tokio::io::copy(&mut rx, &mut wx).await.is_ok() {
        println!("copy message to: {}", stream.peer_addr().unwrap())
    } else {
        eprintln!("faild to copy")
    }
}