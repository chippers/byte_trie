use crate::key::Key;
use crate::node::Node;

mod key;
mod node;

#[derive(Debug)]
pub struct ByteTrie<T> {
    root: Node<T>,
    empty: bool,
}

impl<T> ByteTrie<T> {
    pub fn new() -> Self {
        Self {
            root: Node::new_empty(),
            empty: true,
        }
    }

    pub fn insert(&mut self, key: Key, value: T) {
        if self.empty {
            self.root.key = key;
            self.root.value = Some(value);
            self.empty = false;
        } else {
            self.root.insert(key, Some(value));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
