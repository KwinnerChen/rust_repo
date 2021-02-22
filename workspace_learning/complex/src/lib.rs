#![allow(dead_code)]

use std::{fmt::{Display, Formatter, Result}, ops::Add, write};


/// 表示复数结构
#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T: Add<Output=T>> Complex<T> {
    /// 创建一个复数结构
    /// # Example
    /// ```
    /// use complex::Complex;
    /// let com = Complex::new(0, 0);
    /// assert_eq!(Complex{re:0, im:0}, com);
    ///```
    /// # Panic!
    /// 只能由实现了Add trait的类型创建
    fn new(re: T, im: T) -> Self {
        Self {
            re,
            im,
        }
    }
}

impl<T: Add<Output=T>> Add for Complex<T> {
    type Output = Self;
    fn add(self, other:Self) -> Self::Output {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}+{}i", self.re, self.im)
    }
}


fn show_me(item: impl Display) {
    println!("{}", item);
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone)]
    struct Foo;

    use super::*;
    #[test]
    fn it_works() {
        let complex1 = Complex::new(3, 5);
        let complex2 = Complex::<i32>::default();
        assert_eq!(complex1, complex1 + complex2);
        assert_eq!(complex1, Complex {re:3, im:5});
        println!("{}", complex1);
    }

    #[test]
    fn do_work() {
        show_me("hello rust!");
    }

    #[test]
    fn do_work_2() {
        let mut s1 = String::from("hello");
        let s2 = &mut s1;
        s2.push_str("rust");

        println!("{:?}", s2);
    }

    #[test]
    fn test_work_3() {
        let mut s = String::from("Hello Rust");
        let a_mut_ref = &mut s;
        let a_mut_ref_2 = a_mut_ref;

        a_mut_ref_2.push_str("!");

        println!("{}", &s);
    }
}
