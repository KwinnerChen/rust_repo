use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::str;
use hyper::{Request, Response, Body};
use hyper::rt::{Future, Stream};
use lazy_static::lazy_static;
use crate::shortener::shorten_url;


type UrlDB = Arc<RwLock<HashMap<String, String>>>;
type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;


lazy_static!{
    static ref SHORT_URLS: UrlDB = Arc::new(RwLock::new(HashMap::new()));
}


pub fn url_service(req: Request<Body>) -> BoxFut {
    let reply = req
    .into_body()
    .concat2()
    .map(move |chunk| {
        let c = chunk
        .iter()
        .cloned()
        .collect::<Vec<u8>>();

        let url_to_shorten = str::from_utf8(&c)
        .unwrap();

        let shortened_url = shorten_url(url_to_shorten);

        let _ = SHORT_URLS
        .write()
        .unwrap()
        .insert(shortened_url.to_string(), url_to_shorten.to_string());

        let a = &*SHORT_URLS
        .read()
        .unwrap();

        Response::new(Body::from(format!("{:?}", a)))
    });

    Box::new(reply)
}