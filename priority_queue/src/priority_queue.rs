mod heap;
use heap::Heap;

pub struct PriorityQueue<T: Ord> {
    hp: Heap<T>,
    max: bool,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new(max: bool) -> PriorityQueue<T> {
        PriorityQueue {
            hp: Heap::<T>::new(),
            max,
        }
    }

    pub fn empty(&self) -> bool {
        self.hp.size() == 0
    }

    pub fn size(&self) -> usize {
        self.hp.size()
    }

    pub fn push(&mut self, data: T) {
        self.hp.insert(data, self.max)
    }

    pub fn pop(&mut self) -> Result<(), &str> {
        self.hp.remove(self.max)
    }

    pub fn top(&self) -> Result<&T, &str> {
        self.hp.view()
    }
}
