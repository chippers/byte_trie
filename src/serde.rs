use crate::child::Child;
use crate::keys::BytesKey;
use crate::nodes::AdaptiveNode;
use crate::tries::{BitTrie, ByteTrie, NibbleTrie};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

macro_rules! impl_serialize_root {
    ($trie:ty, $name: expr) => {
        impl<T> Serialize for $trie
        where
            T: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_newtype_struct($name, &self.root)
            }
        }
    };
}

impl_serialize_root!(ByteTrie<T>, "ByteTrie");
impl_serialize_root!(NibbleTrie<T>, "NibbleTrie");
impl_serialize_root!(BitTrie<T>, "BitTrie");

impl<K, T> Serialize for AdaptiveNode<K, T>
where
    K: BytesKey,
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

fn node_or_children<K, T, M>(node: &AdaptiveNode<K, T>, map: &mut M) -> Result<(), M::Error>
where
    K: BytesKey,
    T: Serialize,
    M: SerializeMap,
{
    let is_empty = match &node.child {
        Some(child) => child.is_empty(),
        None => true,
    };

    if is_empty {
        map.serialize_entry(&node.key.to_string(), &node.value)
    } else {
        map.serialize_entry(&node.key.to_string(), &node.child)
    }
}

fn flatten_node<K, T>(node: &AdaptiveNode<K, T>) -> Vec<&AdaptiveNode<K, T>>
where
    K: BytesKey,
    T: Serialize,
{
    if !node.key.get().is_empty() {
        return vec![node];
    }

    match &node.child {
        Some(child) => child.child().iter().fold(Vec::new(), |mut acc, child| {
            if let Some(child) = child {
                acc.append(&mut flatten_node(child));
            }

            acc
        }),
        None => Vec::new(),
    }
}

impl<K, T> Serialize for Child<K, T>
where
    K: BytesKey,
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
