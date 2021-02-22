#![allow(dead_code)]
#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad,
}

#[derive(Debug)]
struct Bag {
    food: Food
}

struct Number<'a> {
    num: &'a u8
}

impl <'a> Number<'a> {
    fn get_num(&self) -> &u8 {
        self.num
    }

    fn set_num(&mut self, new_num: &'a u8) {
        self.num = new_num;
    }
}

fn main() {
    let a = 10;
    let mut num = Number {num: &a};

    num.set_num(&23);

    println!("{:?}", num.get_num());
    
    // let b = Bag {food:Food::Cake};
    // let c = b;  // 结构没有实现copy trait，所有权已转移至变量c
    // let f = move || {
    //     println!("{:?}", b);
    // };

}

