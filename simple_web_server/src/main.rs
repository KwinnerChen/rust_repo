use std::process;
use std::net::TcpListener;
use simple_web_server::*;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        println!("connection is failed: {:?}", err);
        process::exit(1);
    });

    let pool = ThreadPool::new(5);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.excute(|| {
            handle_connection(stream);
        });
    }
}

