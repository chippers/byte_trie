//! `u8` based key implementations.

pub use crate::BytesKey;
use std::fmt;

/// How much of two `BytesKey`s share a prefix
pub enum KeyMatch {
    /// The keys are **exactly** the same
    Exact,
    /// The original key is fully matched, new key has more bytes
    FullSelf(usize),
    /// The new key is fully matched, the original has more bytes
    FullOther(usize),
    /// Both keys match partially, and both have additional bytes
    Partial(usize),
    /// No parts of the new keys match
    None,
}

/// A `u8` based key representing bytes
#[derive(Debug)]
pub struct ByteKey(Vec<u8>);

impl BytesKey for ByteKey {
    fn new(vec: Vec<u8>) -> Self {
        ByteKey(vec)
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        ByteKey(bytes.to_vec())
    }

    fn get(&self) -> &[u8] {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl fmt::Display for ByteKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_for_each(|b| write!(f, "{:02x}", b))
    }
}

/// A `u8` based key representing nibbles
#[derive(Debug)]
pub struct NibbleKey(Vec<u8>);

impl BytesKey for NibbleKey {
    fn new(nibble_vec: Vec<u8>) -> Self {
        NibbleKey(nibble_vec)
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        NibbleKey(
            bytes
                .iter()
                .fold(Vec::with_capacity(bytes.len() * 2), |mut vec, byte| {
                    vec.push(byte >> 4);
                    vec.push(byte & 0x0F);
                    vec
                }),
        )
    }

    fn get(&self) -> &[u8] {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl fmt::Display for NibbleKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_for_each(|b| write!(f, "{:x}", b))
    }
}

/// A `u8` based key representing bits
#[derive(Debug)]
pub struct BitKey(Vec<u8>);

impl BytesKey for BitKey {
    fn new(bit_vec: Vec<u8>) -> Self {
        BitKey(bit_vec)
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        BitKey(
            bytes
                .iter()
                .fold(Vec::with_capacity(bytes.len() * 8), |mut vec, byte| {
                    vec.push(byte & 0x1);
                    vec.push(byte & 0x2);
                    vec.push(byte & 0x4);
                    vec.push(byte & 0x8);
                    vec.push(byte & 0x10);
                    vec.push(byte & 0x20);
                    vec.push(byte & 0x40);
                    vec.push(byte & 0x80);
                    vec
                }),
        )
    }

    fn get(&self) -> &[u8] {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl fmt::Display for BitKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_for_each(|b| write!(f, "{:b}", b))
    }
}
