#![allow(unused_variables)]
#![allow(dead_code)]


use crate_test::*;


fn main() {
    // 
    let vec1 = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    let vec2 = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    // send_messages_by_channel(vec1.clone());
    // multi_transmitter(vec1.clone(), vec2.clone());
    // mutex_api();
    // thread_mutex();

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 70,
                lable: String::from("ok")
            }),
            Box::new(SelectButton{
                width: 70,
                height: 60,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no")
                ]
            })
        ]
    };

    screen.run();
}