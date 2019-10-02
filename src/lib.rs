//!
//! IDEA ONE
//! A trie that uses indexes into a flat Vec<T> where T is the single element of a
//! sequence
//! 
//! NodeEdge
//! for "cat" and "cow"
//! [a, c, o, t, w]
//! asking for "c" would find the index of "c"
//! [a, c, o, t, w]
//!     ^
//! which gives a NodeEdge or Trie.nodes index which gives indexes into sorted_seq of all children
//! [a, c, o, t, w]
//!  ^     ^
//! and again for each child recursivly
//! [a, c, o, t, w]
//!           ^  ^ o's 
//!          a's
//! <br>
//! IDEA TWO
//! HashMap key of hashed (&[T], T) where [T] is the previous elements of the sequence and T is the current
//! element of &[T]. The value stared are nodes that have hashmap indexes to child nodes?
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use fnv::FnvHasher;

mod node;
use node::{Node};

pub(crate) fn fnv_hash<T>(to_hash: (&[T], &T)) -> u64 
where 
    T: Hash,
{
    let mut hasher = FnvHasher::default();
    to_hash.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone)]
pub struct Trie<T> {
    children: HashMap<u64, Node<T>>
}
impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
        }
    }
}

impl<T> Trie<T> 
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Trie { children: HashMap::new(), }
    }

    fn _insert(&mut self, seq: &[T], val: Option<T>, mut idx: usize) {
        if let Some(val) = val {
            let key = fnv_hash((&seq[..idx], &val));

            if self.children.contains_key(&key) {
                // add new keys to Node.children vec
                // we just checked its in here
                let node = self.children.get_mut(&key).unwrap();
                node.update_children(seq, idx);
                idx += 1;
                if let Some(next) = seq.get(idx) {
                    self._insert(seq, Some(next.clone()), idx);
                    return;
                }
            }

            let terminal = seq.len() == idx + 1;
            let node = Node::new(val, &seq, idx, terminal);
            self.children.insert(key, node);
            idx += 1;
            if let Some(next) = seq.get(idx) {
                self._insert(seq, Some(next.clone()), idx)
            }
        }
    }

    pub fn insert(&mut self, seq: &[T]) {
        if let Some(first) = seq.first() {
            let key = fnv_hash((seq, first));
            if self.children.contains_key(&key) {

            }
            self._insert(seq, Some(first.clone()), 0)
        }
    }

    pub fn find(&self, seq_key: &[T]) -> Vec<T> {
        let i = seq_key.len() - 1;
        let key = fnv_hash((&seq_key[..i], &seq_key[i]));

        let mut res = Vec::new();
        if let Some(node) = self.children.get(&key) {
            res.push(node.val.clone());
            for n in node.iter(self) {
                res.push(n.val.clone());
            }
        }
        res
    }

    // pub fn iter(&self) -> TrieIter<T> {
    //     TrieIter {
    //         trie: self,
    //         current: self.children,
    //         children: self.children,
    //         idx: 0,
    //     }
    // }
}

// pub struct TrieIter<'a, T> {
//     trie: &'a Trie<T>,
//     current: Option<&'a Node<T>>,
//     children: Vec<u64>,
//     idx: u64,
// }
// impl<'a, T> Iterator for TrieIter<'a, T> {
//     type Item = &'a Node<T>;
//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.insert(&['c', 'a', 't']);
        // trie.insert(&['c', 'o', 'w']);
        let found = trie.find(&['c']);
        println!("{:?}", found);
    }
}
