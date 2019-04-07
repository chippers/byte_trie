use crate::key::{Key, KeyMatch};
use core::fmt::Debug;
use std::{fmt, mem};

pub(crate) enum Child<T> {
    _1(Box<[Option<Node<T>>; 1]>),
    _2(Box<[Option<Node<T>>; 2]>),
    _4(Box<[Option<Node<T>>; 4]>),
    _8(Box<[Option<Node<T>>; 8]>),
    _16(Box<[Option<Node<T>>; 16]>),
    _32(Box<[Option<Node<T>>; 32]>),
    _64(Box<[Option<Node<T>>; 64]>),
    _128(Box<[Option<Node<T>>; 128]>),
    _256(Box<[Option<Node<T>>; 256]>),
}

impl<T> Child<T> {
    fn new(size: usize) -> Self {
        match size {
            1 => Self::new_1(),
            2 => Self::new_2(),
            4 => Self::new_4(),
            8 => Self::new_8(),
            16 => Self::new_16(),
            32 => Self::new_32(),
            64 => Self::new_64(),
            128 => Self::new_128(),
            256 => Self::new_256(),
            _ => Self::new_1(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Child::_1(_) => 1,
            Child::_2(_) => 2,
            Child::_4(_) => 4,
            Child::_8(_) => 8,
            Child::_16(_) => 16,
            Child::_32(_) => 32,
            Child::_64(_) => 64,
            Child::_128(_) => 128,
            Child::_256(_) => 256,
        }
    }

    fn consume(&mut self) -> Self {
        unimplemented!()
    }
}

impl<T: fmt::Debug> fmt::Debug for Child<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Child::_1(c) => c.fmt(f),
            Child::_2(c) => c.fmt(f),
            Child::_4(c) => c.fmt(f),
            Child::_8(c) => c.fmt(f),
            Child::_16(c) => c.fmt(f),
            Child::_32(c) => c.fmt(f),
            Child::_64(c) => c.fmt(f),
            Child::_128(c) => c.fmt(f),
            Child::_256(c) => c.fmt(f),
        }
    }
}

fn smallest_upgrade(size: usize, lhs: u8, rhs: u8) -> usize {
    loop {
        match next_size(size) {
            None => return 256,
            Some(next) => {
                let lhs_diff = lhs as usize % next;
                let rhs_diff = rhs as usize % next;
                if lhs_diff != rhs_diff {
                    return next;
                }
            }
        }
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
pub(crate) struct Node<T> {
    pub(crate) key: Key,
    pub(crate) value: Option<T>,
    pub(crate) child: Child<T>,
}

impl<T> Node<T> {
    fn new(key: Key, value: Option<T>, child: Child<T>) -> Self {
        unimplemented!()
    }

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
    /// double the child node size, or simply just add a new child.
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
                let new_size = smallest_upgrade(self.child.size(), self.key[0], key[0]);
                let old_node = self.shrink(idx, Child::new(new_size));

                self.add_child_node(old_node);
                self.add_child(key.split_off(idx), value);
            }

            // This node will become parent to both current and new node
            KeyMatch::None => {
                let new_size = smallest_upgrade(self.child.size(), self.key[0], key[0]);
                let old_node = self.shrink(0, Child::new(new_size));

                self.add_child_node(old_node);
                self.add_child(key, value);
            }
        }
    }

    pub(crate) fn add_child(&mut self, key: Key, value: Option<T>) {
        unimplemented!()
    }

    pub(crate) fn add_child_node(&mut self, child: Self) {
        unimplemented!()
    }
}

macro_rules! child_new_init {
    ($new_fn:ident, $init:expr) => {
        impl<T> Child<T> {
            fn $new_fn() -> Self {
                $init
            }
        }
    };
}

child_new_init!(new_1, Child::_1(Box::new([None])));
child_new_init!(new_2, Child::_2(Box::new([None, None])));
child_new_init!(new_4, Child::_4(Box::new([None, None, None, None])));
child_new_init!(
    new_8,
    Child::_8(Box::new([None, None, None, None, None, None, None, None]))
);
child_new_init!(
    new_16,
    Child::_16(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None
    ]))
);
child_new_init!(
    new_32,
    Child::_32(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None
    ]))
);
child_new_init!(
    new_64,
    Child::_64(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None
    ]))
);
child_new_init!(
    new_128,
    Child::_128(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None
    ]))
);
child_new_init!(
    new_256,
    Child::_256(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None
    ]))
);
