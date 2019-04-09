use crate::key::Key;
use crate::node::Node;

mod child;
mod key;
mod node;
#[cfg(feature = "serde")]
mod serde;

#[derive(Debug)]
pub struct ByteTrie<T> {
    root: Node<T>,
}

/// A specialized byte based trie with many different sized nodes
///
/// Currently insert only, as that's the only functionality that I needed.
impl<T> ByteTrie<T> {
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, key: Key, value: T) {
        self.root.insert(key, Some(value));
    }
}

impl<T> Default for ByteTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}
