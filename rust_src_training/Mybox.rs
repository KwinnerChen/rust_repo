#![allow(unused_variables)]

/// 自定义一个实现Deref的智能指针
/// 
use std::ops::Deref;

struct Mybox<T> (T);

impl<T> Mybox<T> {
    fn new(x: T) -> Mybox<T> {
        Mybox(x)
    }
}

impl<T> Deref for Mybox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


fn hello(name: &str) {
    println!("{}", name);
}

fn main() {
    let s = Mybox::new(String::from("hello"));
    hello(&s);
}