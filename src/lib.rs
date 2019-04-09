use crate::key::Key;
use crate::node::Node;
use std::fmt::Debug;

mod child;
mod key;
mod node;

#[derive(Debug)]
pub struct ByteTrie<T>
where
    T: Debug,
{
    root: Node<T>,
}

/// A specialized byte based trie with many different sized nodes
///
/// Currently insert only, as that's the only functionality that I needed.
impl<T> ByteTrie<T> {
    pub fn new() -> Self {
        Self {
            root: Node::new_empty(),
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
