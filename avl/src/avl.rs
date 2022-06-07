use std::cmp::Ordering::{Equal, Greater, Less};

type HeapNode<T> = Option<Box<AVLNode<T>>>;

struct AVLNode<T: Ord> {
    value: T,
    left: HeapNode<T>,
    right: HeapNode<T>,
    height: isize,
}

impl<T: Ord> AVLNode<T> {
    fn new(value: T) -> AVLNode<T> {
        AVLNode {
            value,
            left: None,
            right: None,
            height: 0,
        }
    }
}

pub struct AVL<T: Ord> {
    root: HeapNode<T>,
    num_elements: usize,
}

impl<T: Ord> AVL<T> {
    pub fn new() -> AVL<T> {
        AVL {
            root: None,
            num_elements: 0,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        false // not implemented yet!
    }

    pub fn is_empty(&self) -> bool {
        self.num_elements == 0
    }

    pub fn empty(&mut self) {
        self.num_elements = 0;
    }

    pub fn insert(&mut self, value: T) {}

    pub fn remove(&mut self, value: &T) {}

    pub fn print(&self) {}

    pub fn find_min(&self) -> &T {}

    pub fn find_max(&self) -> &T {}
}
