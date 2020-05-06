use std::io;
use rand::Rng;

fn main()
{
    println!("说出你想的一个数字：");

    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1, 101);

    io::stdin()
        .read_line(&mut guess)
        .expect("读取输入出错了！");

    println!("你想的是：{}", guess);
    println!("随机数：{}", secret_num);
}
