#![allow(unused_imports)]
#![allow(dead_code)]


use training::*;
mod threadpool;
use threadpool::threadpool::test_threadpool;


fn main() {
    // add_trait_test();

    // rc_test();

    // #[derive(Debug)]
    // struct Person<T: Into<String>> {
    //     name: T,
    // }

    // impl<T> Person<T> where T: Into<String> {
    //     fn new(s: T) -> Person<T> {
    //         Person {
    //             name: s
    //         }
    //     }
    // }

    // let p = Person::new("chen".to_string());

    // println!("{:?}", p);
    
    test_threadpool();
}