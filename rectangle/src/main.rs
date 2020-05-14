struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10, 
        height: 40,
    };

    let square1 = Rectangle::square(20);

    println!("rect1的面积：{}", rect1.area());
    println!("rect1能否容下rect2:{}", rect1.can_hold(&rect2));
    println!("square1的面积：{}", square1.area());
}