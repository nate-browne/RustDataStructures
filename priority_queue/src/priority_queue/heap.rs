use std::cmp::Ordering::{Greater, Less};

pub struct Heap<T: Ord> {
    vec: Vec<T>,
}

impl<T: Ord> Heap<T> {
    fn get_parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn get_left(i: usize) -> usize {
        2 * i + 1
    }

    fn get_right(i: usize) -> usize {
        2 * i + 2
    }

    fn heapify_down_max(&mut self, i: usize) {
        let left: usize = Heap::<T>::get_left(i);
        let right: usize = Heap::<T>::get_right(i);

        let mut max: usize = i;

        // Find the largest between the left and right children
        if left < self.size() {
            match self.vec[left].cmp(&self.vec[i]) {
                Greater => max = left,
                _ => (),
            }
        }

        if right < self.size() {
            match self.vec[right].cmp(&self.vec[max]) {
                Greater => max = right,
                _ => (),
            }
        }

        // If we're not the max, swap elements and recursively heapify down
        if max != i {
            self.vec.swap(i, max);
            self.heapify_down_max(max);
        }
    }

    fn heapify_down_min(&mut self, i: usize) {
        let left: usize = Heap::<T>::get_left(i);
        let right: usize = Heap::<T>::get_right(i);

        let mut min: usize = i;

        // Find the smallest between the left and right children
        if left < self.size() {
            match self.vec[left].cmp(&self.vec[i]) {
                Less => min = left,
                _ => (),
            }
        }

        if right < self.size() {
            match self.vec[right].cmp(&self.vec[min]) {
                Less => min = right,
                _ => (),
            }
        }

        // If we're not the min, swap elements and recursively heapify down
        if min != i {
            self.vec.swap(i, min);
            self.heapify_down_min(min);
        }
    }

    fn heapify_up_max(&mut self, i: usize) {
        // Make sure we're not the root and that the parent violates
        // the heap property
        if i > 0 {
            let parent = Heap::<T>::get_parent(i);
            match self.vec[parent].cmp(&self.vec[i]) {
                Less => {
                    self.vec.swap(i, parent);
                    self.heapify_up_max(parent);
                }
                _ => (),
            }
        }
    }

    fn heapify_up_min(&mut self, i: usize) {
        // Make sure we're not the root and that the parent violates
        // the heap property
        if i > 0 {
            let parent = Heap::<T>::get_parent(i);
            match self.vec[parent].cmp(&self.vec[i]) {
                Greater => {
                    self.vec.swap(i, parent);
                    self.heapify_up_min(parent);
                }
                _ => (),
            }
        }
    }

    pub fn new() -> Heap<T> {
        Heap { vec: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    pub fn insert(&mut self, data: T, max: bool) {
        self.vec.push(data);

        let ind: usize = self.size() - 1; // element is added to back of vector

        // restore heap property
        if max {
            self.heapify_up_max(ind);
        } else {
            self.heapify_up_min(ind);
        }
    }

    pub fn remove(&mut self, max: bool) -> Result<(), &str> {
        if self.size() == 0 {
            return Err("Cannot remove from empty heap.");
        }

        let last_ind = self.size() - 1;

        // Replace first element with last and remove the last element
        self.vec.swap(0, last_ind);
        self.vec.pop();

        if max {
            self.heapify_down_max(0);
        } else {
            self.heapify_down_min(0);
        }
        Ok(())
    }

    pub fn view(&self) -> Result<&T, &str> {
        if self.size() == 0 {
            return Err("Cannot top from empty heap.");
        } else {
            return Ok(&self.vec[0]);
        }
    }
}
