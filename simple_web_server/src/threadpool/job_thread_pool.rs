use std::thread;
use std::sync::{mpsc, Arc, Mutex};


/// Job是一个类型别名
/// 
/// 代表一个可在线程间传送的闭包
type Job = Box<dyn FnOnce() + Send + 'static>;


/// 线程间传递的信息，包含任务和停止信号
enum Message {
    NewJob(Job),
    Terminate,
}


/// 工作线程的包装，包含线程分配的id和线程的Option枚举
/// Option枚举为了能在最终join时获取线程的所有权
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// 创建并运行一个worker线程
    /// 
    /// 当管道有任务时线程会立即运行
    /// 
    /// 否则在receiver端阻塞
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // 获取一个任务信息
                let message = reciever.lock().unwrap().recv().unwrap();

                match message {
                    // match需要穷举
                    Message::NewJob(job) => {
                        println!("worker {} got a job; executing.", id);
                        // job是Box指针，函数名也是一种指针，可以直接执行
                        job();
                    },

                    Message::Terminate => {
                        println!("worker {} called terminate.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}


/// 构造线程池
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

/// 为ThreadPool实现drop方法，在跳出作用域时实现线程清理工作
/// 作用域为调用实例ThreadPool的作用域
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("sending terminate message to all workers!");
        // 清理时向所有线程发送停止信号
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for workder in &mut self.workers {
            println!("shutting down worker {}", workder.id);
            // take方法将取出Some中的值，并替换为None，如果是None时不会有动作
            if let Some(thread) = workder.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}