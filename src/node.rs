use std::collections::HashMap;
use std::hash::Hash;

use crate::{fnv_hash, Trie};

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

    pub(crate) fn iter<'a>(&'a self, trie: &'a Trie<T>) -> NodeIter<'a, T> {
        NodeIter {
            map: &trie.children,
            current: Some(self),
            next: None,
            all_kids: Vec::new(),
            first: true,
        }
    }
}

pub(crate) struct NodeIter<'a, T> {
    map: &'a HashMap<u64, Node<T>>,
    current: Option<&'a Node<T>>,
    next: Option<&'a Node<T>>,
    all_kids: Vec<u64>,
    first: bool,
}
impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.current {
            if self.next.is_none() {
                self.all_kids.extend(curr.children.iter().cloned());
                let key = curr.children.first().unwrap();
                let next = self.map.get(key);
                self.next = next;
                return self.current;
            }
            
            if !curr.children.is_empty() {
                self.current = self.next;
                self.all_kids.extend(curr.children.iter().cloned());
                let key = curr.children.first().unwrap();
                self.next = self.map.get(key);
                self.current
            } else {
                None
            }
        } else {
            None
        }
    }
}
