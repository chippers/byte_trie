//! `u8` based trie implementations.

use crate::keys::{BitKey, ByteKey, NibbleKey};
pub use crate::BytesTrie;
use crate::{AdaptiveNode, BytesKey};

/// A `u8` based Trie represented with bytes.
#[derive(Debug)]
pub struct ByteTrie<T> {
    pub(crate) root: AdaptiveNode<ByteKey, T>,
}

impl<T> BytesTrie<T> for ByteTrie<T> {
    fn new() -> Self {
        Self {
            root: AdaptiveNode::default(),
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

/// A `u8` based Trie represented with nibbles.
#[derive(Debug)]
pub struct NibbleTrie<T> {
    pub(crate) root: AdaptiveNode<NibbleKey, T>,
}

impl<T> BytesTrie<T> for NibbleTrie<T> {
    fn new() -> Self {
        Self {
            root: AdaptiveNode::default(),
        }
    }

    fn insert(&mut self, key: &[u8], value: T) {
        let key = NibbleKey::from_bytes(&key);
        self.root.insert(key, Some(value))
    }
}

/// A `u8` based Trie represented with bits.
#[derive(Debug)]
pub struct BitTrie<T> {
    pub(crate) root: AdaptiveNode<BitKey, T>,
}

impl<T> BytesTrie<T> for BitTrie<T> {
    fn new() -> Self {
        Self {
            root: AdaptiveNode::default(),
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
