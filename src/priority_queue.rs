use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

struct QueueItem<T> {
    priority: u32,
    node: T,
}
impl<T> PartialEq for QueueItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl<T> Eq for QueueItem<T> {}

impl<T> PartialOrd for QueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reversed to have a min heap
        Some(self.cmp(other))
    }
}

impl<T> Ord for QueueItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub struct PriorityQueue<T> {
    queue: BinaryHeap<QueueItem<T>>,
    set: HashSet<T>,
}

impl<T> PriorityQueue<T>
where
    T: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            set: HashSet::new(),
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        self.set.contains(item)
    }

    pub fn push(&mut self, item: T, priority: u32) {
        if !self.contains(&item) {
            self.queue.push(QueueItem {
                priority,
                node: item.clone(),
            });
            self.set.insert(item.clone());
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(QueueItem { node, .. }) = self.queue.pop() {
            self.set.remove(&node);
            Some(node)
        } else {
            None
        }
    }
}
