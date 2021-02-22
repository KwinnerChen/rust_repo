#![allow(dead_code)]


use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Option<Rc<Node<T>>>,
    length: usize
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, cur_head: Option<Rc<Node<T>>>) -> Self {
        Self {
            data,
            next: cur_head,
        }
    }
}

impl<T: Copy> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            length: 0
        }
    }

    fn append(&mut self, data: T) {
        let cur_head = self.head.clone();
        let new_node = Node::new(data, cur_head);
        
        self.head = Some(Rc::new(new_node));
        self.length += 1;
    }

    fn get(&mut self) -> Option<T> {
        let cur_head = self.head.clone();
        let result = match cur_head {
            Some(node) => {
                self.length -= 1;
                self.head = node.next.clone();
                Some(node.data) // TODO:返回值应该有更好的处理方式
            },
            None => None,
        };

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_work() {
        let mut list = List::<i32>::new();
        list.append(1);
        list.append(2);
        list.append(3);

        println!("list: {:?}\tlen: {}", list, list.length);
        println!("get a data: {:?}", list.get());
        println!("list: {:?}\tlen: {}", list, list.length);

        list.get();
        list.get();
        list.get();

        println!("list: {:?}\tlen: {}", list, list.length);
    }
}