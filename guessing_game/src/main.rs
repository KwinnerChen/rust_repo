use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    println!("说出你想的一个数字：");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {                                                                  // rust有的无限循环
        let mut guess = String::new();
        io::stdin()                                                         // 标准输入读取的都是字符串
            .read_line(&mut guess)
            .expect("读取输入出错了！");

        let guess: i32 = match guess.trim().parse() {                       // 覆盖变量，进行类型转换
            Ok(num) => num,
            Err(_) => {
                println!("需要输入一个整数哦。。。。");
                continue;
            },
        };   

        println!("你想的是：{}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("猜中了！这个数字就是：{}", secret_num);
                break;
            },
            Ordering::Less => println!("猜小了！"),
            Ordering::Greater => println!("猜大了！"),
        }
    }
}
