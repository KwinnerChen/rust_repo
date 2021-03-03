//! 简易的网址缩短服务器


use log::{info, error};
use std::env;
use hyper::Server;
use hyper::service::service_fn;
use hyper::rt::{self, Future};

mod service;
mod shortener;


fn main() {
    // 设置日子记录级别
    env::set_var("RUST_LOG", "hyperurl=info");
    // 初始化日志记录器
    pretty_env_logger::init();

    let addr = env::args()
    .skip(1)
    .next()
    .unwrap_or("127.0.0.1:3002".to_string());

    let addr = addr.parse().unwrap();

    let server = Server::bind(&addr)
    .serve(|| service_fn(service::url_service))
    .map_err(|e| error!("Server error: {:?}", e));

    info!("URL shortener listening on http://{}", &addr);
    rt::run(server);
}