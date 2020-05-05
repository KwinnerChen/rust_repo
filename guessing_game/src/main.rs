use std::io;

fn main()
{
    println!("说出你想的一个数字：");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取输入出错了！");

    println!("你想的是：{}", guess);
}
