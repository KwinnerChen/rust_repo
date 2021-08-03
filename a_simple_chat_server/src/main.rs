//!使用async-std库实现一个简易的聊天服务器
//!主要是为了熟悉async-std库的练习项目
//!相同的代码可以用tokio来实现;


use a_simple_chat_server::run;

// async_std只有在启用attributes特性时才可以使用async_std：：main这类注释
// 但是async_std默认时关闭该特性的，因为使用注解特性将影响编译事件
fn main() {
    let addr = std::env::args()
    .skip(1)
    .next()
    .unwrap_or("127.0.0.1:8080".to_owned());

    if let Err(e) = run(&addr) {
        eprintln!("{}", e);
    }
}
