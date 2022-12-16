use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct Heap<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    heap: BinaryHeap<HeapItem<T, O>>,
}

impl<T, O> Heap<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T, priority: O) {
        self.heap.push(HeapItem { priority, item });
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.item)
    }

    /// Pops with the value as well as the priority
    pub fn pop_with_priority(&mut self) -> Option<(T, O)> {
        self.heap.pop().map(|item| (item.item, item.priority))
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|item| &item.item)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapItem<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    priority: O,
    item: T,
}

impl<T, O> PartialOrd for HeapItem<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<T, O> Ord for HeapItem<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
