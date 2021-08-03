use std::sync::Arc;
use std::collections::hash_map::{Entry, HashMap};
use async_std::{io::{BufReader, prelude::{BufReadExt}}, net::{TcpListener, TcpStream}, task, sync::Mutex};
use futures::{AsyncWriteExt, FutureExt, SinkExt, StreamExt, select};
use futures::channel::mpsc;


// 该类型别名用于？运算符，遇到错误返回的类型
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;
type Sender<T> = mpsc::UnboundedSender<T>;


// 一个客户端名称和回写发送端的映射容器
// 这里可以定义在broker_loop中，而不用static
// 因为broker_loop只运行一次
lazy_static::lazy_static! {
    static ref PEERS: Arc<Mutex<HashMap<String, Sender<String>>>> = Arc::new(Mutex::new(HashMap::new()));
}


pub fn run(addr: &str) -> Result<()> {
    task::block_on(accept_loop(addr))
}


async fn accept_loop(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("server running on: {:?}", listener.local_addr()?);
    // 可以使用accept方法，置于loop循环中
    let mut incoming = listener.incoming();

    // 这里用了线程并发的思想，使用一个异步任务处理接受的信息，并选择是发送还是创建消息队列等待消息
    let (broker_sender, broker_receiver) = mpsc::unbounded();
    let broker_handler = task::spawn(async move {
        broker_loop(broker_receiver).await
    });

    while let Some(stream) = incoming.next().await {
        // 这里的？运算符相当于unwrap()的作用
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        let sender = broker_sender.clone();
        // 每一个链接使用一个异步任务
        // 类似于线程接口和python的协程任务
        // 任务在后台运行，返回一个句柄用于处理结果
        task::spawn(async move {
            if let Err(e) = connection_loop(stream, sender).await {
                eprintln!("{}", &e)
            }
        });
    }

    // 抛弃向回写代理发送任务的端口
    // 回写代理将推出循环
    drop(broker_sender);
    // 等待回写代理所有任务完成
    broker_handler.await?;

    Ok(())
}


async fn connection_loop(stream: TcpStream, mut broker: Sender<Event>) -> Result<()> {
    // 标准库BufReader的异步版本
    // 由于stream包含读写两端，reader也需要一直读取，所以使用了Arc指针
    // 异步运行时可能是多线程的，所以需要Send和sync，所以使用了Arc而不是Rc
    // 其实可以做一个分离io::split将stream分解为rd和wr，读写两端分别操作
    let stream = Arc::new(stream);
    let reader = BufReader::new(&*stream);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    println!("name = {}", &name);

    // 该通道是一个信道，一端在回写异步任务中，一端在当前链接的异步循环中
    // 当客户端关闭，回写任务关闭，回写端关闭，则链接循环端也将关闭，链接端将不再接受另一客户端向该客户端写入
    let (_shutdown_sender, _shutdown_receiver) = mpsc::unbounded::<Void>();
    // 这里先解析协议第一行，创建所属通道，并常链接等待收取消息
    broker.send(Event::NewPeer {
        name: name.clone(),
        stream: Arc::clone(&stream),
        shutdown: _shutdown_receiver
    }).await?;

    while let Some(line) = lines.next().await {
        let line = line?;
        // line是字节串，所以使用‘：’分割
        let (dest, msg) = match line.find(':') {
            None => continue,
            // 分出名称和消息
            Some(idx) => (&line[..idx], line[idx+1..].trim()),
        };

        let dest = dest.split(',').map(|name| name.trim().to_string()).collect::<Vec<String>>();
        let msg = msg.to_string();

        broker.send(Event::Message {
            from: name.clone(),
            to: dest,
            msg: msg
        }).await?;
    }

    Ok(())
}


async fn connection_write_loop(msg: Receiver<String>, stream: Arc<TcpStream>, shutdown: Receiver<Void>) -> Result<()> {
    // Arc无法移动内部类型的所有权，所以必须是内部类型的引用
    let mut stream = &*stream;
    // 迭代器的fuse方法返回一个Fuse迭代器
    // 该迭代器保证在第一次迭代值为None后的每次迭代值都为None
    let mut message = msg.fuse();
    let mut shutdown = shutdown.fuse();
    
    loop {
        // 这里futures和tokio的select行为有点差异
        // futures的select需要一个Fuse类型，转化函数fuse定义在FutureExt中；文档中解释select在多个future完成时随机选择一个执行分支
        // 但是从test中的结果看，两个更倾向于返回第一个
        // tokio中的select只是需要一个异步块；执行最先返回的一个分支，多个完成时随机选择一个。从相同的返回函数测试看tokio与描述的更贴切
        // 看来还是tokio更完善
        select! {
            msg = message.next().fuse() => match msg {
                Some(msg) => stream.write_all(msg.as_bytes()).await?,
                None => break,
            },

            void = shutdown.next().fuse() => match void {
                Some(_) => (),
                None => break,
            }
        }
    }

    Ok(())
}


// 定义一个事件event，代表发送信息的两种情况
// 头一次发送信息和非第一次发送信息
#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
        shutdown: Receiver<Void>
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    }
}


// 一个符号标记
#[derive(Debug)]
struct Void;


/// 用户代理，用来处理接受的信息
///
/// 将在accept_loop启动后以异步任务的方式启动，通过一个无界通道接受任务，就像并发变成中的线程一样
async fn broker_loop(mut events: Receiver<Event>) -> Result<()> {
    // 一个回写任务句柄的容器
    let mut writers = Vec::new();
    // broker_sender关闭后，推出循环
    while let Some(event) = events.next().await {
        match event {
            Event::Message {from, to, msg} => {
                for addr in to {
                    if let Some(peer) = PEERS.clone().lock().await.get_mut(&addr) {
                        let msg = format!("from {} {}", from, msg);
                        peer.send(msg).await?;
                    }
                }
            }
            Event::NewPeer { name, stream, shutdown } => {
                match PEERS.clone().lock().await.entry(name) {
                    Entry::Occupied(..) => (),
                    Entry::Vacant(entry) => {
                        // 该通道用于向回写任务发送任务
                        // 一端在回写异步任务中，一端保存在映射中
                        let (client_sender, client_receiver) = mpsc::unbounded();
                        entry.insert(client_sender);
                        // 每个回写任务都是一个长tcp链接，单独一个异步任务
                        // 就像单独的线程一样，但是异步任务由程序调度而不是系统，占用资源更少
                        // 也是通过一个无界通道接受任务
                        let handle = task::spawn(async move {
                            if let Err(e) = connection_write_loop(client_receiver, stream, shutdown).await {
                                eprintln!("{}", e);
                            }
                        });

                        writers.push(handle);
                    }
                }
            }
        }
    }

    // 这里删除所有回写异步任务的发送端
    // 以使回写任务推出循环
    PEERS.lock().await.clear();

    // 等待所有回写任务完成
    for writer in writers {
        writer.await;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use futures::{select, FutureExt};
    
    async fn return_num(n: i32) -> i32 {
        n
    }

    #[async_std::test]
    async fn test_select() {
        let res = select! {
            a_res = return_num(1).fuse() => a_res,
            b_res = return_num(2).fuse() => b_res,
        };

        println!("select result is: {}", res);
    }
}