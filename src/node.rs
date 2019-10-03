use std::collections::HashMap;
use std::hash::Hash;

use crate::{fnv_hash, Trie, PreHashedMap};

#[derive(Debug, Clone)]
pub struct NodeEdge {
    val_idx: usize,
    node_idx: usize,
}

impl NodeEdge {

}

#[derive(Debug, Clone, Eq)]
pub(crate) struct Node<T> {
    pub(crate) val: T,
    children: Vec<u64>,
    child_size: usize,
    terminal: bool,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T> Node<T> 
where
    T: Eq + Hash + Clone,
{
    pub(crate) fn new(val: T, seq: &[T], idx: usize, terminal: bool) -> Node<T> {
        let i = idx + 1;
        let mut children = Vec::new();

        if let Some(ele) = seq.get(i) {
            children.push(fnv_hash((&seq[..i], ele)));
        }

        Self {
            val,
            children,
            child_size: 0,
            terminal,
        }
    }

    pub(crate) fn update_children(&mut self, seq: &[T], idx: usize) {
        let i = idx + 1;
        if let Some(ele) = seq.get(i) {
            let key = fnv_hash((&seq[..i], ele));
            if !self.children.contains(&key) {
                self.children.push(key);
            }
        }
    }
    /// Depth first iteration of a node and its children.
    pub(crate) fn iter<'a>(&'a self, trie: &'a Trie<T>) -> NodeIter<'a, T> {
        NodeIter {
            map: &trie.children,
            current: self,
            next: None,
            all_kids: Vec::new(),
        }
    }
}

pub(crate) struct NodeIter<'a, T> {
    map: &'a PreHashedMap<u64, Node<T>>,
    current: &'a Node<T>,
    next: Option<&'a Node<T>>,
    all_kids: Vec<u64>,
}
impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_none() {
            self.all_kids.extend(self.current.children.iter().cloned());

            let key = self.all_kids.remove(0);
            let next = self.map.get(&key);
            self.next = next;
            return self.next;
        } else {
            // next is always Some
            self.current = self.next.unwrap();
            // all kids will be empty for the end case
            self.all_kids.extend(self.current.children.iter().cloned());

            if self.all_kids.is_empty() { return None };

            let key = self.all_kids.remove(0);
            self.next = self.map.get(&key);
            self.next
        }
    }
}
