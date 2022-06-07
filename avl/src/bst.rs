use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::Debug;
use std::cmp::max;

type HeapNode<T> = Option<Box<BSTNode<T>>>;

struct BSTNode<T: Ord + Debug> {
    value: T,
    left: HeapNode<T>,
    right: HeapNode<T>,
    height: isize,
    balance: isize,
}

impl<T: Ord + Debug> BSTNode<T> {
    fn new(value: T) -> BSTNode<T> {
        BSTNode {
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
                    None => self.right = Some(Box::from(BSTNode::new(value))),
                }
            },
            Less => {
                match self.left {
                    Some(ref mut node) => node.insert(value),
                    None => self.left = Some(Box::from(BSTNode::new(value))),
                }
            }
        }
        self.balance();
    }

    fn height(node: &HeapNode<T>) -> isize {
        match node {
            Some(n) => n.height,
            None => -1,
        }
    }

    fn balance(&mut self) {
        let l_height = BSTNode::height(&self.left);
        let r_height = BSTNode::height(&self.right);


        self.height = max(l_height, r_height) + 1;
        self.balance = l_height - r_height;
    }

    fn find_min(&self) -> &T {
        match &self.left {
            Some(node) => node.find_min(),
            None => &self.value,
        }
    }

    fn find_max(&self) -> &T {
        match &self.right {
            Some(node) => node.find_max(),
            None => &self.value,
        }
    }

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

    fn contains(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Equal => true,
            Greater => match &self.right {
                Some(node) => node.contains(value),
                None => false,
            },
            Less => match &self.left {
                Some(node) => node.contains(value),
                None => false,
            }
        }
    }
}

pub struct BST<T: Ord + Debug> {
    root: HeapNode<T>,
    size: isize,
}

impl<T: Ord + Debug> BST<T> {

    pub fn new() -> BST<T> {
        BST {
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
        match &self.root {
            Some(node) => node.contains(value),
            None => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::from(BSTNode::new(value))),
        }
        self.size += 1;
        self.check();
    }

    pub fn print(&self) {
        match &self.root {
            Some(node) => node.print(),
            None => println!("Tree is empty.",)
        };
    }

    pub fn find_min(&self) -> Result<&T, &str> {
        match &self.root {
            Some(node) => Ok(node.find_min()),
            None => Err("Tree is empty."),
        }
    }

    pub fn find_max(&self) -> Result<&T, &str> {
        match &self.root {
            Some(node) => Ok(node.find_max()),
            None => Err("Tree is empty."),
        }
    }
}
