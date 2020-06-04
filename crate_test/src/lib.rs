use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::sync::{Mutex, Arc};


// 定义一个迭代器
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


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


fn thread_maker(transmitter: Sender<String>, vec: Vec<String>) {
    thread::spawn(move || {
        let current_threadid = thread::current().id();
        for val in vec {
            transmitter.send(format!("{:?}-{}", current_threadid, val)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn consumer(receiver: Receiver<String>) {
    for val in receiver {
        println!("got: {}", val);
    }
}


// 使用一个通道多次发送消息，主线程接受，
pub fn send_messages_by_channel(vec: Vec<String>) {
    let (tx, rx) = mpsc::channel();

    thread_maker(tx, vec);

    consumer(rx);
}


pub fn multi_transmitter(vec1: Vec<String>, vec2: Vec<String>) {
    let (tx, rx) = mpsc::channel();

    let tx_clone = mpsc::Sender::clone(&tx);

    thread_maker(tx, vec1);
    thread_maker(tx_clone, vec2);

    consumer(rx);
}


pub fn mutex_api() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}


// 使用Arc和Mutex在多线程中共享数据
pub fn thread_mutex() {
    // 初始化一个计数器，可以线程共享，且每个线程都有所有权
    let counter = Arc::new(Mutex::new(0));
    // 初始化一个线程容器
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}


// 定义一个trait，使所有元素具有统一接口
pub trait Draw {
    fn draw(&self);
}

// 定义屏幕结构，结构包含所有实现了Draw trait的对象
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 绑定结构伤的方法
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 一下开始定义具体窗口具体元素类型
pub struct Button {
    pub width: i32,
    pub height: i32,
    pub lable: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("there is a Buttong");
    }
}

pub struct SelectButton {
    pub width: i32,
    pub height: i32,
    pub options: Vec<String>,
}

impl Draw for SelectButton {
    fn draw(&self) {
        println!("there is a selectbutton");
    }
}