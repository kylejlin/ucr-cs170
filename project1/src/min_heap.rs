use std::collections::BinaryHeap as MaxHeap;

#[derive(Debug, Clone)]
pub struct MinHeap<T> {
    heap: MaxHeap<InvertOrder<T>>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self {
            heap: MaxHeap::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.heap.push(InvertOrder(value));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|v| v.0)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|v| &v.0)
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvertOrder<T>(pub T);

impl<T: PartialOrd> PartialOrd for InvertOrder<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for InvertOrder<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
