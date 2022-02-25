pub struct Node {
    data: isize,
    link: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn unshift(&mut self, data: isize) {
        let new_node = Node {
            data,
            link: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn push(&mut self, data: isize) {
        let new_node = Node { data, link: None };
        match self.head {
            None => self.head = Some(Box::new(new_node)),
            Some(ref mut head) => {
                let mut p = head;
                loop {
                    match p.link {
                        None => {
                            p.link = Some(Box::new(new_node));
                            break;
                        }
                        Some(ref mut next) => p = next,
                    }
                }
            }
        }
    }

    pub fn get(&self, index: isize) -> Option<isize> {
        match self.head {
            None => return None,
            Some(ref top) => {
                let mut p = top;
                let mut i = 0;
                loop {
                    if i == index {
                        return Some(p.data);
                    }
                    match p.link {
                        None => return None,
                        Some(ref link) => p = link,
                    }
                    i += 1;
                }
            }
        }
    }
}

pub fn sec_02() {
    let mut list = List::new();
    list.push(100);
    list.push(200);
    list.unshift(10);
    list.unshift(20);
    println!("{}", list.get(0).unwrap());
    println!("{}", list.get(1).unwrap());
    println!("{}", list.get(2).unwrap());
    println!("{}", list.get(3).unwrap());
}

// pub fn node(data: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
//     Some(Box::new(Node { data, link }))
// }

// pub enum Node {
//     Empty,
//     Cons(i64, Box<Node>),
// }
//
// use Node::{Cons, Empty};
//
// pub fn node(v: i64, link: Box<Node>) -> Box<Node> {
//     Box::new(Cons(v, link))
// }
