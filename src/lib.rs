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
use std::hash::{Hash, Hasher};

use fnv::FnvHasher;

mod noop_hash;
pub use noop_hash::PreHashedMap;
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
    starts: Vec<u64>,
    children: PreHashedMap<u64, Node<T>>
}
impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self {
            children: PreHashedMap::default(),
            starts: Vec::default(),
        }
    }
}

impl<T> Trie<T> 
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Trie { children: PreHashedMap::default(), starts: Vec::default(), }
    }

    fn _insert(&mut self, seq: &[T], val: Option<T>, mut idx: usize) {
        if let Some(val) = val {
            let key = fnv_hash((&seq[..idx], &val));
            // let key = (&seq[..idx], &val);

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
            let key = fnv_hash((&[], first));
            if !self.starts.contains(&key) { self.starts.push(key) };
            self._insert(seq, Some(first.clone()), 0)
        }
    }

    pub fn find(&self, seq_key: &[T]) -> Vec<T> {
        let i = seq_key.len() - 1;
        let key = fnv_hash((&seq_key[..i], &seq_key[i]));
        // let key = (&seq_key[..i], &seq_key[i]);
        let mut res = Vec::new();
        if let Some(node) = self.children.get(&key) {
            res.push(node.val.clone());
            for n in node.walk(self) {
                res.push(n.val.clone());
            }
        }
        res
    }

    pub fn iter(&self) -> TrieIter<T> {
        TrieIter {
            trie: self,
            current: None,
            starts: &self.starts,
            children: Vec::default(),
            idx: 0,
            next_idx: 0,
        }
    }
}

pub struct TrieIter<'a, T> {
    trie: &'a Trie<T>,
    current: Option<&'a Node<T>>,
    starts: &'a [u64],
    children: Vec<u64>,
    idx: usize,
    next_idx: usize,
}
impl<'a, T> Iterator for TrieIter<'a, T> 
where
    T: Clone + Eq + Hash,
{
    type Item = &'a Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            // this bails us out of the iteration
            let key = self.starts.get(self.idx)?;
            self.current = Some(self.trie.children.get(&key)?);
            self.idx += 1;
            // we know its there
            self.children = self.current.unwrap()
                .walk(self.trie)
                .map(|n| n.key)
                .collect::<Vec<_>>();

            self.current
        } else {
            let key = self.children[self.next_idx];
            self.current = self.trie.children.get(&key);
            self.next_idx += 1;

            if self.next_idx >= self.children.len() {
                self.next_idx = 0;
                let curr = self.current.take();
                curr
            } else {
                self.current
            }
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct Found<T> {
//     roll_back: Vec<usize>,
//     temp: Vec<T>,
//     collected: Vec<Vec<T>>,
// }

// impl<T: Clone + PartialEq> Found<T> {
//     fn new() -> Self {
//         Self {
//             roll_back: vec![],
//             temp: vec![],
//             collected: vec![],
//         }
//     }

//     pub fn as_collected(&self) -> Vec<&[T]> {
//         self.collected
//             .iter()
//             .map(|seq| seq.as_slice())
//             .collect::<Vec<_>>()
//     }

//     fn push_val(&mut self, t: T) {
//         self.temp.push(t);
//     }

//     fn branch_end_continue(&mut self) {
//         self.collected.push(self.temp.clone());
//     }

//     fn branch_split(&mut self, key: &T)
//     where
//         T: std::fmt::Debug,
//     {
//         if let Some(idx) = self.temp.iter().position(|item| key == item) {
//             let (start, end) = self.temp.split_at(idx + 1);
//             self.temp = start.to_vec();
//         }
//     }

//     fn branch_end(&mut self) {
//         self.collected.push(self.temp.clone());
//         // remove last element
//         self.temp.pop();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_find() {
        let mut trie = Trie::new();
        trie.insert(&['c', 'a', 't']);
        trie.insert(&['c', 'o', 'w']);
        let found = trie.find(&['c']);
        println!("{:?}", found);
    }

    #[test]
    fn trie_iter() {
        let mut trie = Trie::new();
        trie.insert(&['c', 'a', 't']);
        trie.insert(&['c', 'o', 'w']);
        for n in trie.iter() {
            println!("{:?}", n);
        }
    }
}
