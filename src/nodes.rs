//! `u8` based node implementations.

use crate::child::{Child, MAX_CHILD_SIZE};
use crate::keys::KeyMatch;
pub use crate::AdaptiveNode;
use crate::BytesKey;

/// The size of a child that does not exist.
///
/// This is very important for `next_size`,
/// `AdaptiveNode::smallest_ancestor_size`, and `Child::size` as careless
/// changing of this size could cause infinite looping in
/// `AdaptiveNode::smallest_ancestor_size`.  
const NO_CHILD: usize = 0;

impl<K: BytesKey, V> AdaptiveNode<K, V> {
    /// Create a new `AdaptiveNode` form a key and value
    pub fn new(key: K, value: Option<V>) -> Self {
        Self {
            key,
            value,
            child: None,
        }
    }

    /// Insert a key into the node.
    ///
    /// This may cause the node to shrink key size, split into an empty parent,
    /// increase the child node size, or simply just add a new child.
    pub fn insert(&mut self, key: K, value: Option<V>) {
        if self.child.is_none() && self.value.is_none() && self.key.get().is_empty() {
            self.key = key;
            self.value = value;
        } else {
            self.insert_node(Self::new(key, value));
        }
    }

    /// Insert a node into a node.
    ///
    /// This may cause the node to shrink key size, split into an empty parent,
    /// increase the child node size, or simply just add a new child.
    pub fn insert_node(&mut self, mut new: Self) {
        match self.key.compare(&new.key) {
            // We've seen this full key before, it's the same edge - replace it
            KeyMatch::Exact => self.value = new.value,

            // New node will be a child of current node
            KeyMatch::FullSelf(idx) => {
                new.key = K::new(new.key.get_mut().split_off(idx));
                self.add_child_node(new)
            }

            // New node will become the parent to the current node
            KeyMatch::FullOther(idx) => {
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
        let size = self.smallest_ancestor_size(&new, idx);

        let current_node = self.replace_to(idx, None, Some(Child::new(size)));
        let new_node = new.replace_to(idx, None, Some(Child::new(size)));

        self.add_child_node(current_node);
        self.add_child_node(new_node);
    }

    /// Find the smallest child size for an ancestor that can fit both child hashes
    fn smallest_ancestor_size(&self, other: &Self, hash_idx: usize) -> usize {
        let lhs = self.key.get()[hash_idx] as usize;
        let rhs = other.key.get()[hash_idx] as usize;
        let mut size = self.child.as_ref().map(Child::size).unwrap_or(NO_CHILD);

        // `next_size` is guaranteed to not return the same number, preventing an infinite loop.
        // we know this because of the specific sized used in `Child::size` and `next_size`.
        // the only other setting of sizes is the line above in the `unwrap_or` where we define
        // the default size for not having a child.
        loop {
            let next = next_size(size);
            if next == MAX_CHILD_SIZE || ((lhs % next) != (rhs % next)) {
                return next;
            }

            size = next;
        }
    }

    // We know by here that the child key has at least 1 byte
    fn add_child_node(&mut self, child: Self) {
        if self.child.is_none() {
            self.child = Some(Child::new(1));
        }

        let current_child = self.child.as_mut().unwrap();
        let slot = current_child.calculate_slot(child.key.get()[0]);

        match current_child.at(slot) {
            Some(existing) => existing.insert_node(child),
            None => current_child.put(slot, child),
        }
    }

    /// Shrink the `Node` to the key index and return the excess as a new node.
    fn replace_to(&mut self, to: usize, value: Option<V>, child: Option<Child<K, V>>) -> Self {
        let excess = Self {
            key: K::new(self.key.get_mut().split_off(to)),
            value: self.value.take(),
            child: self.child.take(),
        };

        self.key.get_mut().shrink_to_fit();
        self.value = value;
        self.child = child;
        excess
    }
}

impl<K: BytesKey, V> Default for AdaptiveNode<K, V> {
    fn default() -> Self {
        Self::new(K::new(Vec::new()), None)
    }
}

/// Find the next valid child size based on the current size
pub(crate) fn next_size(size: usize) -> usize {
    match size {
        0 => 1,
        1 => 2,
        2 => 4,
        4 => 8,
        8 => 16,
        16 => 32,
        32 => 64,
        64 => 128,
        _ => MAX_CHILD_SIZE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_size_cannot_infinitely_loop() {
        let mut size = NO_CHILD;
        while size < MAX_CHILD_SIZE {
            let next = next_size(size);
            assert_ne!(size, next);
            size = next;
        }
    }
}
