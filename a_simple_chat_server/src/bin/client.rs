use async_std::{io::{BufReader, prelude::BufReadExt, stdin}, net::TcpStream};
use futures::{AsyncReadExt, AsyncWriteExt, FutureExt, StreamExt, select};


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[async_std::main]
async fn main() -> Result<()>{
    let addr = std::env::args()
    .skip(1)
    .next()
    .unwrap_or("127.0.0.1:8080".to_string());

    try_run(&addr).await
}


async fn try_run(addr: &str) -> Result<()>{
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = stream.split();

    let mut lines_from_server = BufReader::new(reader).lines().fuse();
    let mut lines_from_stdin = BufReader::new(stdin()).lines().fuse();
    
    loop {
        select! {
            line = lines_from_server.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    println!("{}", line);
                },
                None => break,
            },

            line = lines_from_stdin.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                },
                None => break
            }
        }
    }

    Ok(())
}