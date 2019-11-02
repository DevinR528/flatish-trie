// use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::fmt::Debug;
use crate::{make_key, Trie, PreHashedMap};

#[derive(Debug, Clone, Eq)]
pub struct Node<T> {
    pub(crate) key: u64,
    pub(crate) val: T,
    pub(crate) children: Vec<u64>,
    pub(crate) child_size: usize,
    pub(crate) terminal: bool,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T> Node<T> 
where
    T: Eq + Hash + Clone + Debug,
{
    pub(crate) fn new(val: T, seq: &[T], idx: usize, terminal: bool) -> Node<T> {
        let key = make_key((&seq[..idx], &seq[idx]));
        let i = idx + 1;
        let mut children = Vec::new();
        if let Some(ele) = seq.get(i) {
            children.push(make_key((&seq[..i], ele)));
        }
        Self {
            key,
            val,
            children,
            child_size: 0,
            terminal,
        }
    }

    pub(crate) fn as_value(&self) -> &T {
        &self.val
    }

    pub(crate) fn to_value(&self) -> T {
        self.val.clone()
    }

    pub(crate) fn is_terminal(&self) -> bool {
        self.terminal
    }

    pub(crate) fn child_len(&self) -> usize {
        self.children.len()
    }

    pub(crate) fn remove_child(&mut self, key: &u64) -> bool {
        if let Some(idx) = self.children.iter().position(|c| c == key) {
            self.children.remove(idx);
            if self.child_len() > 0 { self.child_size -= 1 };
            true
        } else {
            false
        }
    }

    pub(crate) fn children<'b, 'a: 'b>(&'a self, map: &'a PreHashedMap<u64, Node<T>>) -> Vec<&'b Node<T>> {
        self.children.iter().map(|key| map.get(key).unwrap()).collect()
    }

    pub(crate) fn update_children(&mut self, seq: &[T], idx: usize) {
        let i = idx + 1;
        if let Some(ele) = seq.get(i) {
            let key = make_key((&seq[..i], ele));
            if !self.children.contains(&key) {
                self.child_size += 1;
                self.children.push(key);
            }
        }
    }
    /// Depth first iteration of a node and its children.
    pub(crate) fn walk<'a>(&'a self, trie: &'a Trie<T>) -> NodeIter<'a, T> {
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
    // TODO try using VecDeque
    all_kids: Vec<u64>,
}
impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // return first child
        if self.next.is_none() {
            self.all_kids.extend(self.current.children.iter().cloned());

            if !self.all_kids.is_empty() {
                let key = self.all_kids.remove(0);
                let next = self.map.get(&key);
                self.next = next;
                self.next
            } else {
                None
            }
            
        // iterate depth first through children
        } else {
            // next is always Some
            self.current = self.next.unwrap();
            // all kids will be empty for the end case
            self.all_kids.splice(0..0, self.current.children.iter().rev().copied());

            if self.all_kids.is_empty() { return None };

            let key = self.all_kids.remove(0);
            self.next = self.map.get(&key);
            self.next
        }
    }
}
