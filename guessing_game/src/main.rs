use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    println!("说出你想的一个数字：");

    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1, 101);

    io::stdin()                                                         // 标准输入读取的都是字符串
        .read_line(&mut guess)
        .expect("读取输入出错了！");

    let guess: i32 = guess.trim().parse().expect("需要输入一个数字！");    // 覆盖变量，进行类型转换

    println!("你想的是：{}", guess);
    println!("随机数：{}", secret_num);

    match guess.cmp(&secret_num) {
        Ordering::Equal => println!("猜中了！"),
        Ordering::Less => println!("猜小了！"),
        Ordering::Greater => println!("猜大了！"),
    }
}
