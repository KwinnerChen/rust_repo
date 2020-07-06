#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]


use rand::prelude::*;
use std::ops::Add;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;


#[derive(Debug)]
struct Point<T>
    where T: Add<Output=T>
{
    x: T,
    y: T,
}

impl<T> Add for Point<T>
    where T: Add<Output=T>
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output
    {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn add_trait_test() {
    let p1 = Point {x: 1, y: 2};
    // let v = vec![1,2,3];
    let mut s = String::from("hello");
    
    let mut y = &mut s;
    let z = &mut y;

    y.push('w');
    
    let mut rng = rand::thread_rng();
    let mut v:Vec<i32>  = (1..100).collect();
    v.shuffle(&mut rng);
    v.sort();
    v.reverse();
    // println!("{:?}", v);

    let set1 = (1..100).collect::<HashSet<_>>();
    let set2 = (2..100).collect::<HashSet<_>>();
    
    println!("{}", &set2.is_subset(&set1));
}


#[derive(Debug)]
struct Owner {
    name: String
}

impl Drop for Owner {
    fn drop(&mut self) {
        println!("{} be droped", self.name);
    }
}

#[derive(Debug)]
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

pub fn rc_test() {
    let gadget_owner = Rc::new(
        Owner {
            name: String::from("Gadget Man")
        }
    );

    let gadget1 = Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    };

    let gadget2 = Gadget {
        id: 2,
        owner: gadget_owner.clone(),
    };

    drop(gadget_owner);

    println!("gadget1: {:?}", gadget1);
    println!("gadget2: {:?}", gadget2);
}
