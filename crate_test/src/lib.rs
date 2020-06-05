use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::sync::{Mutex, Arc};


/// 定义一个迭代器，用于计数
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

////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////

/// 创建一个线程
fn thread_maker(transmitter: Sender<String>, vec: Vec<String>) {
    thread::spawn(move || {
        let current_threadid = thread::current().id();
        for val in vec {
            transmitter.send(format!("{:?}-{}", current_threadid, val)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

/// 通道的消费者
fn consumer(receiver: Receiver<String>) {
    for val in receiver {
        println!("got: {}", val);
    }
}


/// 使用一个通道多次发送消息，主线程接受，
pub fn send_messages_by_channel(vec: Vec<String>) {
    let (tx, rx) = mpsc::channel();

    thread_maker(tx, vec);

    consumer(rx);
}

/// 通过clone多个管道的发送端，共不同线程使用
pub fn multi_transmitter(vec1: Vec<String>, vec2: Vec<String>) {
    let (tx, rx) = mpsc::channel();

    let tx_clone = mpsc::Sender::clone(&tx);

    thread_maker(tx, vec1);
    thread_maker(tx_clone, vec2);

    consumer(rx);
}

/// 单线城中Mutex的演示
pub fn mutex_api() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}


/// 使用Arc和Mutex在多线程中共享数据
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
////////////////////////////////////////////////////////////////////////////////

/// 定义一个trait，使所有元素具有统一接口
pub trait Draw {
    fn draw(&self);
}

/// 定义屏幕结构，结构包含所有实现了Draw trait的对象
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


//////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 一个模拟的博文发布crate
pub struct Post {
    // 博文审核状态
    // 提供一种思路，使用枚举
    // 面向对象思想，其实可以用一个数值表示
    state: Option<Box<dyn State>>,
    // 博文内容
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }

    /// 此方法用于博文的审核操作，并改变博文的审核状态
    /// take方法将原变量只为None，并提取值赋给新的变量
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    /// 向博文中添加内容
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// 通过状态state接口，间接返回结构内容
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    /// 批准审批
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

/// trait对象，类似面向对象的抽象基类。
trait State {
    /// 每一个状态结构体都应包含一个审批入口
    /// 该接口用于改返回一个新的状态值
    /// 注意这里self不是引用，这会时老的状态失去所有权而被放弃，
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    /// 同意审批接口
    fn approve(self: Box<Self>) -> Box<dyn State>;

    /// 定义一个简介获取内容的接口，
    /// 内容与状态相关
    /// 默认返回空字符串，周偶状态为published时才返回真确内容
    /// 当published时返回的时post实例的一部分，所有需要声明声明post和&str声明周期一致
    /// 
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// 结构体没有数据，只有操作，表示状态
// 表示草稿状态
struct Draft {}

/// trait State的具体实现
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // 草稿状态，同意审批则还是草稿状态，返回自身
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// 结构体没有数据，只有操作
// 表示审批中
struct PendingReview {}

impl State for PendingReview {
    // 审批中申请审批则状态还是审批中，返回自身
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 同意审批后，状态应该为可发表状态
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 状态可发表
struct  Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 覆写content方法， 返回实际内容
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
