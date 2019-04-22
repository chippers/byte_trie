#![doc(html_root_url = "https://docs.rs/byte_trie/0.3.0")]

use crate::child::Child;
use crate::keys::KeyMatch;
use std::fmt::Display;

mod child;
pub mod keys;
pub mod nodes;
#[cfg(feature = "serde")]
mod serde;
pub mod tries;

/// Represents a trie with node keys having the maximum size of `u8`.
///
/// Meant to be used for any type that can be represented within a  single
/// `u8`, such as a byte, a nibble, or bit.
pub trait BytesTrie<T> {
    /// Create an empty trie
    fn new() -> Self;

    /// Insert a key and value to the trie structure
    fn insert(&mut self, key: &[u8], value: T);
}

/// A byte-driven representation of an adaptive compressed trie node.
///
/// Largest key's value can be a `u8`, here supplied by `BytesKey`.
#[derive(Debug)]
pub struct AdaptiveNode<K: BytesKey, V> {
    pub(crate) key: K,
    pub(crate) value: Option<V>,
    pub(crate) child: Option<Child<K, V>>,
}

/// A way of representing a `BytesNode` key `u8` inside a `Vec<u8>`.
///
/// Keeps track of `u8` representation and serialization presentation.
pub trait BytesKey: Display {
    /// Create a new key from a `Vec<u8>` already in the proper representation
    fn new(vec: Vec<u8>) -> Self;

    /// Create a new key from a `Vec<u8>` representing full bytes
    fn from_bytes(bytes: &[u8]) -> Self;

    /// Get an immutable slice reference to the underlying `Vec<u8>`.
    fn get(&self) -> &[u8];

    /// Get a mutable reference to the underlying `Vec<u8>`
    fn get_mut(&mut self) -> &mut Vec<u8>;

    /// Compare the shared prefix length of two keys
    fn compare(&self, other: &Self) -> KeyMatch {
        let prefix = self
            .get()
            .iter()
            .zip(other.get().iter())
            .take_while(|(&lhs, &rhs)| lhs == rhs)
            .count();

        let self_len = self.get().len();
        let other_len = other.get().len();

        if prefix == self_len && prefix == other_len {
            KeyMatch::Exact
        } else if prefix == self_len && other_len > self_len {
            KeyMatch::FullSelf(prefix)
        } else if prefix == other_len && self_len > other_len {
            KeyMatch::FullOther(prefix)
        } else if prefix > 0 {
            KeyMatch::Partial(prefix)
        } else {
            KeyMatch::None
        }
    }
}

/// A "prelude" for users of the `bytes_trie` crate
pub mod prelude {
    pub use crate::keys::{BitKey, ByteKey, NibbleKey};
    pub use crate::nodes::AdaptiveNode;
    pub use crate::tries::{BitTrie, ByteTrie, NibbleTrie};
    pub use crate::{BytesKey, BytesTrie};
}
