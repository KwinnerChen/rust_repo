use std::thread;
use rand::Rng;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;


#[derive(Debug)]
enum Level {
    A,
    B,
    C,
    D,
    E,
    None,
}

#[derive(Debug)]
struct Student {
    name: String,
    score: i32,
    level: Level,
}

#[derive(Debug)]
enum Task {
    Task(Student),
    Terminate,
}

fn init_vec(students: &mut Vec<Student>) {
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        students.push(
            Student {
                name: i.to_string(),
                score: rng.gen_range(0, 101),
                level: Level::None,
            }
        )
    }
}

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Task>,
}

impl ThreadPool {
    fn new(size: i32) -> ThreadPool {
        let (sender, reciever) = mpsc::channel::<Task>();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
        for i in 0..size {
            let reciever_clone = Arc::clone(&reciever);
            let thread_handle = thread::spawn(move || {
                loop {
                    let task = reciever_clone.lock().unwrap().recv().unwrap();
                    match task {
                        Task::Task(student) => {
                            let mut student = student;
                            if student.score >= 90 {
                                student.level = Level::A;
                            } else if student.score >= 80 {
                                student.level = Level::B;
                            } else if student.score >= 70 {
                                student.level = Level::C;
                            } else if student.score >=60 {
                                student.level = Level::D;
                            } else {
                                student.level = Level::E;
                            }
                            println!("thread {} is running, {:?}\n", i, student);
                        },

                        Task::Terminate => break,
                    }
                }
            });

            workers.push(thread_handle);
            }

        ThreadPool {
            workers,
            sender,
        }

        
    }

    fn excute(&self, task: Task) {
        self.sender.send(task).unwrap();
    }
}


pub fn test_threadpool() {
    let mut students_vec: Vec<Student> = Vec::new();
    
    init_vec(&mut students_vec);

    let thread_pool = ThreadPool::new(3);

    for student in students_vec {
        thread_pool.excute(Task::Task(student));
    }

    for _ in 0..3 {
        thread_pool.excute(Task::Terminate);
    }

    for t in thread_pool.workers {
        t.join().unwrap();
    }
    println!("done!");
}