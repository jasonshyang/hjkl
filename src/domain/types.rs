use std::collections::VecDeque;

const DEFAULT_QUEUE_LEN: usize = 16;

/// Direction for moving in the buffer
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Forward,
    Backward,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}

/// Fixed capacity buffer queue
pub struct BoundedQueue<T>(VecDeque<T>);

impl<T> Default for BoundedQueue<T> {
    fn default() -> Self {
        BoundedQueue::new(DEFAULT_QUEUE_LEN)
    }
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        BoundedQueue(VecDeque::with_capacity(capacity))
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn push(&mut self, item: T) {
        // Remove oldest item if at capacity
        if self.0.len() == self.0.capacity() {
            self.0.pop_front();
        }
        self.0.push_back(item);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    pub fn last(&self) -> Option<&T> {
        self.0.back()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn reverse_iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.0.iter().rev()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
