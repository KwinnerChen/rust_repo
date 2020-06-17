use std::io::prelude::*;
use std::fs;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};


/// 请求相应接口
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    stream.read(&mut buffer).unwrap();
    println!("requests: {}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let status_line_ok = "HTTP/1.1 200 OK\r\n\r\n";
    let status_line_404 = "HTTP/1.1 400 Not Found\r\n\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        (status_line_ok, "./static/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (status_line_ok, "./static/hello.html")
    } else {
        (status_line_404, "./static/404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


/// 构造线程池
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// 创建线程池
    /// 
    /// 线程池中线程的数量
    /// 
    /// # Panics
    /// 
    /// ‘new’ 函数在size为0时会panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, reciever) = mpsc::channel();
        // reciever需要多线程用于所有权，并多线程访问
        let reciever = Arc::new(Mutex::new(reciever));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    /// 将闭包包装为
    pub fn excute<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = reciever.lock().unwrap().recv().unwrap();
                println!("worker {} got a job; executing.", id);
                // job是Box指针，函数名也是一种指针，可以直接执行
                job();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}


// Job是一个
type Job = Box<dyn FnOnce() + Send + 'static>;