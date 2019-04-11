use crate::key::{BitKey, ByteKey, BytesKey, NibbleKey};
use crate::node::BytesNode;

pub trait BytesTrie<T> {
    fn new() -> Self;
    fn insert(&mut self, key: &[u8], value: T);
}

#[derive(Debug)]
pub struct ByteTrie<T> {
    pub(crate) root: BytesNode<ByteKey, T>,
}

/// A specialized byte based trie with many different sized nodes
///
/// Currently insert only, as that's the only functionality that I needed.
impl<T> BytesTrie<T> for ByteTrie<T> {
    fn new() -> Self {
        Self {
            root: BytesNode::default(),
        }
    }

    fn insert(&mut self, key: &[u8], value: T) {
        let key = ByteKey::from_bytes(&key);
        self.root.insert(key, Some(value));
    }
}

impl<T> Default for ByteTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NibbleTrie<T> {
    pub(crate) root: BytesNode<NibbleKey, T>,
}

impl<T> BytesTrie<T> for NibbleTrie<T> {
    fn new() -> Self {
        Self {
            root: BytesNode::default(),
        }
    }

    fn insert(&mut self, key: &[u8], value: T) {
        let key = NibbleKey::from_bytes(&key);
        self.root.insert(key, Some(value))
    }
}

#[derive(Debug)]
pub struct BitTrie<T> {
    pub(crate) root: BytesNode<BitKey, T>,
}

/// A specialized byte based trie with many different sized nodes
///
/// Currently insert only, as that's the only functionality that I needed.
impl<T> BytesTrie<T> for BitTrie<T> {
    fn new() -> Self {
        Self {
            root: BytesNode::default(),
        }
    }

    fn insert(&mut self, key: &[u8], value: T) {
        let key = BitKey::from_bytes(&key);
        self.root.insert(key, Some(value));
    }
}

impl<T> Default for BitTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}
