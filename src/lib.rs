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

impl<T> ByteTrie<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self {
            root: Node::new_empty(),
        }
    }

    pub fn insert(&mut self, key: Key, value: T) {
        self.root.insert(key, Some(value));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
