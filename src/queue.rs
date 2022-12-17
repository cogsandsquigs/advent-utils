use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct Queue<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    queue: BinaryHeap<QueueItem<T, O>>,
}

impl<T, O> Queue<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T, priority: O) {
        self.queue.push(QueueItem { priority, item });
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop().map(|item| item.item)
    }

    /// Pops with the value as well as the priority
    pub fn pop_with_priority(&mut self) -> Option<(T, O)> {
        self.queue.pop().map(|item| (item.item, item.priority))
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.peek().map(|item| &item.item)
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Truncates the queue to the given length. If the queue is shorter than the given length, it
    /// will be left unchanged.
    /// TODO: This is a bit of a hack, but it works for now. It would be nice to have a way to
    /// truncate a BinaryHeap.
    pub fn truncate(&mut self, len: usize) {
        let mut new_queue = BinaryHeap::new();

        if self.queue.len() <= len {
            return;
        }

        for _ in 0..len {
            if let Some(item) = self.queue.pop() {
                new_queue.push(item);
            }
        }
        self.queue = new_queue;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueueItem<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    priority: O,
    item: T,
}

impl<T, O> PartialOrd for QueueItem<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<T, O> Ord for QueueItem<T, O>
where
    T: PartialEq + Eq + Clone,
    O: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
