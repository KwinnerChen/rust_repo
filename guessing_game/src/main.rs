use std::io;
fn main() {
    println!("想一个数字！");
    println!("下面输入你想的数字：");

    let mut guess = String::new();

    // 不够ruster
    // let input = io::stdin();
    // let result = input.read_line(&mut guess);
    // result.expect("读取输入失败");

    io::stdin()
        .read_line(&mut guess)
        .expect("读取输入失败！");

    println!("你想的数字是：{}", guess);
}
