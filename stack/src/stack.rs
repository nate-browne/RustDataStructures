pub struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            stack: Vec::new(),
        }
    }

    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn top(&self) -> &T {
        match self.stack.last() {
            Some(val) => val,
            None => panic!("Called `top` on an empty stack"),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.stack.push(elem);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
