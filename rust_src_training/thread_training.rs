use std::thread;
use std::sync::mpsc;


fn main() {
    let (tx, rx) = mpsc::channel();
    let rx1 = mpsc::Receiver::clone(&rx);

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("got {}", received);
}