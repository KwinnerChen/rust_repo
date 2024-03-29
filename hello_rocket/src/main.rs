#[macro_use]
extern crate rocket;


// rocket的路由分两部分，handler函数注解一部分，一般就简化为“/”
// 另外一部分在构建服务的时候mount挂载
// 主要在mount里设置，所以handler函数注解一般简化为“/”
// 但是也可以根据实际需求来分配一个确定的路由，例如路径参数
#[get("/")]
async fn index() -> &'static str {
    "hello rocket!"
}


#[get("/<name>")]
async fn hello(name: &str) -> String {
    format!("Hello {}", name)
}


// rocke的分为异步和同步两种模式
// 同步使用rocket：：launch注解一个非main函数，rocketbuilder不需要调用launch和await
// 异步使用rocket：：main注解main函数，rocketbuilder需要调用launch和await
#[rocket::main]
async fn main() {
    let result = rocket::build()
    .mount("/", routes![index])
    .mount("/hello", rocket::routes![index])
    .mount("/hi", routes![index])
    .mount("/hello", routes![hello])
    .launch()
    .await;

    if let Err(e) = result {
        eprintln!("shutdown beacause: {:?}", e);
    }
}