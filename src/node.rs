use crate::child::{Child, MAX_CHILD_SIZE};
use crate::key::{Key, KeyMatch};

fn smallest_upgrade(mut size: usize, lhs: u8, rhs: u8) -> usize {
    loop {
        match next_size(size) {
            None => return MAX_CHILD_SIZE,
            Some(next) => {
                let lhs_diff = lhs as usize % next;
                let rhs_diff = rhs as usize % next;
                if lhs_diff != rhs_diff {
                    return next;
                } else {
                    size = next
                }
            }
        };
    }
}

fn next_size(size: usize) -> Option<usize> {
    match size {
        0 => Some(1),
        1 => Some(2),
        2 => Some(4),
        4 => Some(8),
        8 => Some(16),
        16 => Some(32),
        32 => Some(64),
        64 => Some(128),
        128 => Some(256),
        _ => None,
    }
}

#[derive(Debug)]
pub(crate) struct Node<T> {
    pub(crate) key: Key,
    pub(crate) value: Option<T>,
    pub(crate) child: Option<Child<T>>,
}

impl<T> Node<T> {
    pub(crate) fn new(key: Key, value: Option<T>, child: Option<Child<T>>) -> Self {
        Self { key, value, child }
    }

    /// Shrink the `Node` to the key index and return the excess as a new node.
    fn replace_to(&mut self, to: usize, value: Option<T>, child: Option<Child<T>>) -> Self {
        let excess = Self {
            key: self.key.split_off(to),
            value: self.value.take(),
            child: self.child.take(),
        };

        self.key.shrink_to_fit();
        self.value = value;
        self.child = child;
        excess
    }

    /// Insert a key into the node.
    ///
    /// This may cause the node to shrink key size, split into an empty parent,
    /// increase the child node size, or simply just add a new child.
    pub(crate) fn insert(&mut self, key: Key, value: Option<T>) {
        if self.key.is_empty() && self.value.is_none() && self.child.is_none() {
            self.key = key;
            self.value = value;
        } else {
            self.insert_node(Self::new(key, value, None));
        }
    }

    fn insert_node(&mut self, mut new: Self) {
        match KeyMatch::compare(&self.key, &new.key) {
            // We've seen this full key before, it's the same edge - replace it
            KeyMatch::Exact => self.value = new.value,

            // New node will be a child of current node
            KeyMatch::FullOriginal(idx) => {
                new.key = new.key.split_off(idx);
                self.add_child_node(new)
            }

            // New node will become the parent to the current node
            KeyMatch::FullNew(idx) => {
                let current_node = self.replace_to(idx, new.value, new.child);
                self.add_child_node(current_node);
            }

            // We need to create an ancestor to parent current and new node
            KeyMatch::Partial(idx) => self.insert_ancestor(new, idx),

            // We need to create an ancestor to parent current and new node
            KeyMatch::None => self.insert_ancestor(new, 0),
        }
    }

    // If we are here we know that the keys have at least `idx` byte each
    fn insert_ancestor(&mut self, mut new: Self, idx: usize) {
        let (current_hash, new_hash) = (self.key[idx], new.key[idx]);

        let current_size = smallest_upgrade(self.child_size(), current_hash, new_hash);
        let current_node = self.replace_to(idx, None, Some(Child::new(current_size)));

        let new_size = smallest_upgrade(new.child_size(), current_hash, new_hash);
        let new_node = new.replace_to(idx, None, Some(Child::new(new_size)));

        self.add_child_node(current_node);
        self.add_child_node(new_node);
    }

    // We know by here that the child key has at least 1 byte
    fn add_child_node(&mut self, child: Self) {
        if self.child.is_none() {
            self.child = Some(Child::new(1));
        }

        let current_child = self.child.as_mut().unwrap();
        let slot = current_child.slot(child.key[0]);

        match current_child.at(slot) {
            Some(existing) => existing.insert_node(child),
            None => current_child.put(slot, child),
        }
    }

    fn child_size(&self) -> usize {
        self.child.as_ref().map(Child::size).unwrap_or(0)
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self::new(Vec::new(), None, None)
    }
}
