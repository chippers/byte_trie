use crate::child::Child;
use crate::node::Node;
use crate::ByteTrie;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::fmt;

struct Hex<'a>(&'a [u8]);

impl<'a> Hex<'a> {
    fn new<T>(node: &'a Node<T>) -> Hex<'a>
    where
        T: Serialize,
    {
        Hex(&node.key)
    }
}

impl<'a> fmt::Display for Hex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_for_each(|b| write!(f, "{:02x}", b))
    }
}

impl<T> Serialize for ByteTrie<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("ByteTrie", &self.root)
    }
}

impl<T> Serialize for Node<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        flatten_node(&self)
            .iter()
            .try_for_each(|n| node_or_children(n, &mut map))?;

        map.end()
    }
}

fn node_or_children<T, M>(node: &Node<T>, map: &mut M) -> Result<(), M::Error>
where
    T: Serialize,
    M: SerializeMap,
{
    let hex = Hex::new(&node).to_string();

    if node.child.child().iter().all(Option::is_none) {
        map.serialize_entry(&hex, &node.value)
    } else {
        map.serialize_entry(&hex, &node.child)
    }
}

fn flatten_node<T>(node: &Node<T>) -> Vec<&Node<T>>
where
    T: Serialize,
{
    if !node.key.is_empty() {
        return vec![node];
    }

    node.child
        .child()
        .iter()
        .fold(Vec::new(), |mut acc, child| {
            if let Some(child) = child {
                acc.append(&mut flatten_node(child));
            }

            acc
        })
}

impl<T> Serialize for Child<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        self.child()
            .iter()
            .filter_map(|c| c.as_ref().map(flatten_node))
            .try_for_each(|nodes| {
                nodes
                    .iter()
                    .try_for_each(|node| node_or_children(node, &mut map))
            })?;

        map.end()
    }
}
