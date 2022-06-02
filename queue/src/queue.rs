use std::collections::VecDeque;

pub struct Queue<T> {
    queue: VecDeque<T>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            queue: VecDeque::new(),
        }
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn front(&self) -> &T {
        match self.queue.front() {
            Some(val) => val,
            None => panic!("Called `front` on empty queue."),
        }
    }

    pub fn back(&self) -> &T {
        match self.queue.back() {
            Some(val) => val,
            None => panic!("Called `back` on empty queue."),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.queue.push_back(elem);
    }

    pub fn pop(&mut self) {
        self.queue.pop_front();
    }
}
