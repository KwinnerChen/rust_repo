//!使用hyperurl来缩短网址的命令行工具


use quicli::prelude::*;
use structopt::StructOpt;


// const常量在代码内是内联的，没有固定的内存位置
const CONN_ADDR: &str = "127.0.0.1:3002";


#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long="url", short="u")]
    url: String,
    
    #[structopt(flatten)]
    verbosity: Verbosity,
}


fn main() -> CliResult {
    let args = Cli::from_args();
    println!("Shortening: {}", args.url);
    let client = reqwest::Client::new();
    let mut resp = client
    .post(&format!("http://{}", CONN_ADDR))
    .body(args.url)
    .send()?;

    let a = resp.text().unwrap();
    println!("http://{}", a);

    Ok(())
}
