use crate::child::{Child, MAX_CHILD_SIZE};
use crate::key::{Key, KeyMatch};
use core::fmt::Debug;
use std::{fmt, mem};

fn smallest_upgrade(mut size: usize, lhs: u8, rhs: u8) -> usize {
    loop {
        match next_size(size) {
            None => return MAX_CHILD_SIZE,
            Some(next) => {
                let lhs_diff = lhs as usize % next;
                let rhs_diff = rhs as usize % next;
                println!(
                    "size: {} | next: {}, lhs: {} | lhs_diff: {} | rhs: {} | rhs_diff: {}",
                    size, next, lhs, lhs_diff, rhs, rhs_diff
                );
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
pub(crate) struct Node<T>
where
    T: Debug,
{
    pub(crate) key: Key,
    pub(crate) value: Option<T>,
    pub(crate) child: Child<T>,
}

impl<T> Node<T>
where
    T: Debug,
{
    pub(crate) fn new_empty() -> Self {
        Self {
            key: Vec::new(),
            value: None,
            child: Child::new_1(),
        }
    }

    /// Shrink the `Node` to the key index and return the rest.
    fn shrink(&mut self, to: usize, new_child: Child<T>) -> Self {
        let excess = Self {
            key: self.key.split_off(to),
            value: self.value.take(),
            child: mem::replace(&mut self.child, new_child),
        };

        self.key.shrink_to_fit();
        excess
    }

    /// Insert a key into the node.
    ///
    /// This may cause the node to shrink key size, split into an empty parent,
    /// increase the child node size, or simply just add a new child.
    pub(crate) fn insert(&mut self, mut key: Key, value: Option<T>) {
        match KeyMatch::compare(&self.key, &key) {
            // We've seen this full key before, it's the same edge - replace it
            KeyMatch::Exact => self.value = value,

            // New node will be a child of current node
            KeyMatch::FullOriginal(idx) => self.add_child(key.split_off(idx), value),

            // New node will become the parent to the current node
            KeyMatch::FullNew(idx) => {
                let old_node = self.shrink(idx, Child::new_1());
                self.value = value;

                self.add_child_node(old_node);
            }

            // This node will become parent to both current and new node
            KeyMatch::Partial(idx) => {
                // If we are here we can be confident in the first index
                let (hash, new_hash) = (self.key[idx], key[idx]);

                let new_size = smallest_upgrade(self.child.size(), hash, new_hash);
                let old_node = self.shrink(idx, Child::new(new_size));

                self.add_child_node(old_node);
                self.add_child(key.split_off(idx), value);
            }

            // This node will become parent to both current and new node
            KeyMatch::None => {
                // If we are here we can be confident in the first index
                let (hash, new_hash) = (self.key[0], key[0]);

                let new_size = smallest_upgrade(self.child.size(), hash, new_hash);
                let old_node = self.shrink(0, Child::new(new_size));

                self.add_child_node(old_node);
                self.add_child(key, value);
            }
        }
    }

    fn insert_node(&mut self, mut child: Self) {
        match KeyMatch::compare(&self.key, &child.key) {
            // We've seen this full key before, it's the same edge - replace it
            KeyMatch::Exact => self.value = child.value,

            // New node will be a child of current node
            KeyMatch::FullOriginal(idx) => {
                child.key = child.key.split_off(idx);
                self.add_child_node(child)
            }

            // New node will become the parent to the current node
            KeyMatch::FullNew(idx) => {
                let old_node = self.shrink(idx, Child::new_1());
                self.value = child.value;
                self.child = child.child;

                self.add_child_node(old_node);
            }

            // This node will become parent to both current and new node
            KeyMatch::Partial(idx) => {
                // If we are here we can be confident in the first index
                let (hash, new_hash) = (self.key[0], child.key[0]);

                let old_size = smallest_upgrade(self.child.size(), hash, new_hash);
                let old_node = self.shrink(idx, Child::new(old_size));

                let child_size = smallest_upgrade(child.child.size(), hash, new_hash);
                let old_child = child.shrink(idx, Child::new(child_size));

                self.add_child_node(old_node);
                self.add_child_node(old_child);
            }

            // This node will become parent to both current and new node
            KeyMatch::None => {
                // If we are here we can be confident in the first index
                let (hash, new_hash) = (self.key[0], child.key[0]);

                let old_size = smallest_upgrade(self.child.size(), hash, new_hash);
                let old_node = self.shrink(0, Child::new(old_size));

                let child_size = smallest_upgrade(child.child.size(), hash, new_hash);
                let old_child = child.shrink(0, Child::new(child_size));

                self.add_child_node(old_node);
                self.add_child_node(old_child);
            }
        }
    }

    // We know by here that the key has at least 1 byte
    fn add_child(&mut self, key: Key, value: Option<T>) {
        let slot = self.child.slot(key[0]);

        match self.child.at(slot) {
            Some(existing) => existing.insert(key, value),
            None => self.child.put(
                slot,
                Self {
                    key,
                    value,
                    child: Child::new_1(),
                },
            ),
        }
    }

    // We know by here that the child key has at least 1 byte
    fn add_child_node(&mut self, child: Self) {
        let slot = self.child.slot(child.key[0]);

        match self.child.at(slot) {
            Some(existing) => existing.insert_node(child),
            None => self.child.put(slot, child),
        }
    }
}
