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

    unsafe {
        println!("befor add count is: {}", COUNT);
    }
    add_for_count(20);
    unsafe {
        println!("after add count is: {}", COUNT);
    }

    let point1 = Point{x:1, y:3};
    let point2 = Point{x:3, y:4};
    let point3 = Point{x:4, y:7};

    assert_eq!(
        point1 + point2,
        point3
    );
    println!("{:?} + {:?} is {:?}", point1, point2, point3);

    println!("{:?}", macro_init_test());

    Pancakes::hello_macro();
}