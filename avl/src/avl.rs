use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::Debug;
use std::cmp::max;

type HeapNode<T> = Option<Box<AVLNode<T>>>;

const THRESHOLD: isize = 1;

#[derive(Debug)]
struct AVLNode<T: Ord + Debug> {
    value: T,
    left: HeapNode<T>,
    right: HeapNode<T>,
    height: isize,
    balance: isize,
}

impl<T: Ord + Debug> AVLNode<T> {
    fn new(value: T) -> AVLNode<T> {
        AVLNode {
            value,
            left: None,
            right: None,
            height: 0,
            balance: 0,
        }
    }

    fn check(&mut self) {
        match self.left {
            Some(ref mut node) => node.check(),
            None => (),
        }

        match self.right {
            Some(ref mut node) => node.check(),
            None => (),
        }

        self.balance();
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Equal => (), // do nothing with duplicate insertions
            Greater => {
                match self.right {
                    Some(ref mut node) => node.insert(value),
                    None => self.right = Some(Box::from(AVLNode::new(value))),
                }
            },
            Less => {
                match self.left {
                    Some(ref mut node) => node.insert(value),
                    None => self.left = Some(Box::from(AVLNode::new(value))),
                }
            }
        }
        self.balance();
    }

    fn height(&self, node: &HeapNode<T>) -> isize {
        match node {
            Some(n) => n.height,
            None => -1,
        }
    }

    fn balance(&mut self) {
        let l_height = self.height(&self.left);
        let r_height = self.height(&self.right);


        self.height = max(l_height, r_height) + 1;
        self.balance = l_height - r_height;

        if self.balance.abs() > THRESHOLD {
            if self.balance < 0 {
                if self.right.as_ref().unwrap().balance < 0 {
                    self.double_left();
                } else {
                    self.rotate_left();
                }
            } else if self.balance > 0 {
                if self.left.as_ref().unwrap().balance < 0 {
                    self.double_right();
                } else {
                    self.rotate_right();
                }
            }
        }
    }

    fn double_left(&mut self) {}

    fn rotate_left(&mut self) {}

    fn double_right(&mut self) {}

    fn rotate_right(&mut self) {}

    fn print(&self) {
        match &self.left {
            Some(node) => node.print(),
            None => (),
        }
        println!(
            "height: {} balance: {} value: {:?}",
            self.height, self.balance, self.value
        );
        match &self.right {
            Some(node) => node.print(),
            None => (),
        }
    }
}

#[derive(Debug)]
pub struct AVL<T: Ord + Debug> {
    root: HeapNode<T>,
    size: isize,
}

impl<T: Ord + Debug> AVL<T> {

    pub fn new() -> AVL<T> {
        AVL {
            root: None,
            size: 0,
        }
    }

    fn check(&mut self) {
        match self.root {
            Some(ref mut node) => node.check(),
            None => (),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        false // not implemented yet!
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn empty(&mut self) {
        self.size = 0;
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::from(AVLNode::new(value))),
        }
        self.check();
    }

    pub fn remove(&mut self, value: &T) {}

    pub fn print(&self) {
        match &self.root {
            Some(node) => node.print(),
            None => println!("Tree is empty.",)
        };
    }

    pub fn find_min(&self) -> &T {
        panic!("Not implemented yet!");
    }

    pub fn find_max(&self) -> &T {
        panic!("Not implemented yet!");
    }
}
