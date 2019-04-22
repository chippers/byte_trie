use crate::child::{Child, MAX_CHILD_SIZE};
use crate::keys::KeyMatch;
pub use crate::AdaptiveNode;
use crate::BytesKey;

impl<K: BytesKey, V> AdaptiveNode<K, V> {
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
        if self.key.get().is_empty() && self.value.is_none() && self.child.is_none() {
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
        let current_hash = self.key.get()[idx];
        let new_hash = new.key.get()[idx];

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
        let slot = current_child.slot(child.key.get()[0]);

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

    fn child_size(&self) -> usize {
        self.child.as_ref().map(Child::size).unwrap_or(0)
    }
}

impl<K: BytesKey, V> Default for AdaptiveNode<K, V> {
    fn default() -> Self {
        Self::new(K::new(Vec::new()), None)
    }
}

// Find the next child size that fits both new child bucket hashes
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

// Find the next valid child size based on the current size
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
