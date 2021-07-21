use std::{env};

use tokio::{io, net::{TcpListener, TcpStream}};


async fn process(socket: TcpStream) {
    let (mut rs, mut ws) = io::split(socket);

    if io::copy(&mut rs, &mut ws).await.is_err() {
        eprintln!("echo erro!");
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = env::args()
    .skip(1)
    .next()
    .unwrap_or("localhost:6142".to_owned());

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("echo server ruuning on: {}", &addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket).await
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncWriteExt, AsyncReadExt};
    use tokio::select;

    #[test]
    fn test_echo_server() {
        tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            let connect = TcpStream::connect("localhost:6142").await.unwrap();

            let (mut rd, mut wt) = io::split(connect);
            
            tokio::spawn(async move {
                &wt.write_all(b"hello\n").await;
                &wt.write_all(b"world\n").await;
                &wt.shutdown();
            });

            let mut buffer = vec![0;128];
            loop {
                let n = rd.read(&mut buffer).await.unwrap();
                println!("{}", n);
                if n == 0 {
                    break;
                }

                println!("got {}", String::from_utf8_lossy(&buffer[..n]));
            }
        });
    }

    async fn return_num(n: i32) -> i32 {
        n
    }

    #[tokio::test]
    async fn test_select() {
        let res = select! {
            a_res = return_num(1) => a_res,
            b_res = return_num(2) => b_res,
        };

        println!("select result is: {}", res);
    }
}
