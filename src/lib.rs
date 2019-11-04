//!
//! A flattened trie that uses indexes into a flat HashMap<T> where T is the single element of a
//! sequence.
//! 
//! for "cat" and "cow" 
//! {
//!     hash of ([], 'c'): Node { children: [hashes of (['c'], 'a'), (['c], 'o')]},
//!     ([], 'a'): Node { children: [hash of (['c', 'a'], 't')] },
//!     ([], 't'): Node { children: [] },
//!     ([], 'o'): Node { children: [hash of (['c', 'o'], 'w')] },
//!     ([], 'w'): Node { children: [] },
//! }
//! asking for "c" would find the index of "c" using a hash of ([], 'c')
//! [a, c, o, t, w]
//!     ^
//! which gives indexes into the children of 'c' generated by (['c'], 'o') or 
//! (['c'], 'a') respectively.
//! [a, c, o, t, w]
//!  ^     ^
//! and again for each child recursively
//! [a, c, o, t, w]
//!           ^  ^ o's 
//!          a's
//! <br>
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::hash_map::Entry;

mod key;
use key::{key_from_seq, key_at_index};
mod node;
use node::{Node};
mod noop_hash;
pub use noop_hash::PreHashedMap;

#[derive(Debug, Clone)]
pub struct Trie<T> {
    starts: Vec<u128>,
    children: PreHashedMap<u128, Node<T>>,
    /// number of unique items T inserted into the trie.
    len: usize,
}
impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self {
            children: PreHashedMap::default(),
            starts: Vec::default(),
            len: 0,
        }
    }
}

impl<T> Trie<T> 
where
    T: Eq + Hash + Clone + Debug,
{
    pub fn new() -> Self {
        Trie { children: PreHashedMap::default(), starts: Vec::default(), len: 0, }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// TODO make this insert in reverse check if optimizes.
    fn _insert(&mut self, seq: &[T], val: Option<T>, mut idx: usize) {
        if let Some(val) = val {
            let key = key_at_index(idx, seq);

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
                return;
            }

            let terminal = seq.len() == idx + 1;
            let node = Node::new(val, &seq, idx, terminal);
            self.children.insert(key, node);
            self.len += 1;
            // if terminal { return };
            idx += 1;
            if let Some(next) = seq.get(idx) {
                self._insert(seq, Some(next.clone()), idx)
            }
        }
    }
    /// Inserts a `seq` or sequence into the trie.
    ///
    /// # Examples
    ///
    /// ```
    /// use ecs_trie::Trie;
    /// let mut trie = Trie::new();
    /// trie.insert(&['c', 'a', 't']);
    /// trie.insert(&['c', 'o', 'w']);
    /// 
    /// let found = trie.search(&['c']);
    /// 
    /// assert_eq!(
    ///     found.as_collected().as_slice(),
    ///     &[ ['c', 'a', 't'], ['c', 'o', 'w'] ]
    /// );
    /// ```
    pub fn insert(&mut self, seq: &[T]) {
        if let Some(first) = seq.first() {
            let key = key_at_index(0, seq);
            if !self.starts.contains(&key) { self.starts.push(key) };
            self._insert(seq, Some(first.clone()), 0)
        }
    }

    // fn _insert2(
    //     &mut self,
    //     seq: &[T],
    //     val: T,
    //     mut idx: usize,
    //     child_key: Option<u64>,
    // ) {
    //     let key = key_at_index(idx, seq);
    //     let terminal = seq.len() == idx + 1;

    //     if self.children.contains_key(&key)  {
    //         return;
    //     }

    //     if idx != 0 {
    //         if let Some(kid_key) = child_key {
    //             let mut node = Node::new(val, seq, idx, terminal);
    //             node.update_children2(kid_key);
    //             self.children.insert(key, node);
    //             idx -= 1;
    //             if let Some(next) = seq.get(idx) {
    //                 self._insert2(seq, next.clone(), idx, Some(key))
    //             }
    //         } else {
    //             let node = Node::new(val, seq, idx, terminal);
    //             self.children.insert(key, node);
    //             idx -= 1;
    //             if let Some(next) = seq.get(idx) {
    //                 self._insert2(seq, next.clone(), idx, Some(key))
    //             }
    //         }
    //     }
    // }

    // pub fn insert2(&mut self, seq: &[T]) {
    //     if let Some(end) = seq.last() {
    //         self._insert2(seq, end.clone(), seq.len() - 1, None)
    //     }
    // }

    fn _search<'n>(
        map: &PreHashedMap<u128, Node<T>>,
        node: &'n Node<T>,
        seq_key: &[T],
        idx: usize,
        found: &mut Found<T>
    ) {        
        // complete terminal branch no children
        if node.is_terminal() && node.child_len() == 0 {
            found.branch_end();
            return;
        // terminal but children after
        } else if node.is_terminal() {
            found.branch_end_continue();
        }
        // recurs iteratively over children
        for n in node.children(map) {
            found.push_val(n.to_value());
            Trie::_search(map, n, seq_key, idx + 1, found);

            // not terminal but has more than one child, if deeper than single
            // node we need a some way of keeping track of what needs to be removed
            // from temp vec
            if !node.is_terminal() && node.child_len() > 1 {
                found.branch_split(node.as_value());
            }
        }
    }

    /// Returns `true` if `seq_key` is found.
    /// Note the last item in seq_key must be a terminal node.
    /// TODO is this a good idea (terminal node)
    pub fn contains(&self, seq_key: &[T]) -> bool {
        let key = key_from_seq(seq_key);
        let mut term = false;
        if let Some(n) = self.children.get(&key) {
            term = n.is_terminal();
        }
        self.children.contains_key(&key) && term
    }

    /// Returns all of the found sequences, walking
    /// each branch depth first.
    ///
    /// # Examples
    ///
    /// ```
    /// use ecs_trie::Trie;
    /// let mut trie = Trie::new();
    /// trie.insert(&['c', 'a', 't']);
    /// trie.insert(&['c', 'o', 'w']);
    /// 
    /// let found = trie.search(&['c']);
    /// 
    /// assert_eq!(
    ///     found.as_collected().as_slice(),
    ///     &[ ['c', 'a', 't'], ['c', 'o', 'w'] ]
    /// );
    /// ```
    pub fn search(&self, seq_key: &[T]) -> Found<T> {
        let key = key_from_seq(seq_key);

        let mut res = Found::new();
        res.extend(seq_key.iter().cloned());
        if let Some(node) = self.children.get(&key) {
            Trie::_search(&self.children, node, seq_key, 1, &mut res)
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

    /// Returns `true` if terminal node has children.
    fn is_terminal_end (&self, seq: &[T]) -> bool {
        let end_key = key_from_seq(seq);
        if let Some(node) = self.children.get(&end_key) {
            node.child_len() > 0
        } else {
            panic!("is stem ish failed bug")
        }
    }

    /// Returns true if seq contains a terminal node anywhere
    /// except the last node.
    fn contains_terminal(&self, seq: &[T]) -> bool {
        seq.iter()
            .enumerate()
            .any(|(i, _)| {
                // every whole seq will be terminal but we only care about
                // the middle bits.
                if i == seq.len() - 1 { return false };

                let key = key_at_index(i, seq);
                if let Some(n) = self.children.get(&key) {
                    n.is_terminal()
                } else {
                    // TODO what to do if node not found
                    // at this point its a bug becasue we have already
                    // checked if trie contains seq
                    panic!("trie mutated when it shouldn't, bug")
                }
            })
    }

    /// Returns index in seq and key to first safe non terminal node anywhere.
    fn contains_terminal_with_key(&self, seq: &[T]) -> Option<(usize, u128)> {
        if seq.iter().enumerate()
            .any(|(i, _)| {
                // every whole seq will be terminal but we only care about
                // the middle bits.
                if i == seq.len() - 1 { return false };

                let key = key_at_index(i, seq);
                if let Some(n) = self.children.get(&key) {
                    n.is_terminal()
                } else {
                    // TODO what to do if node not found
                    // at this point its a bug becasue we have already
                    // checked if trie contains seq
                    panic!("trie mutated when it shouldn't, bug")
                }
            })
        {
            let mut key = key_from_seq(seq);
            seq.iter()
                .enumerate()
                .rev()
                .skip(1)
                .find(|(i, _)| {
                    let key = key_at_index(*i, seq);
                    if let Some(node) = self.children.get(&key) {
                        node.is_terminal()
                    } else {
                        false
                    }
                })
                .map(|(i, _)| (i, key))
        } else {
            None
        }
    }

    /// Clears the `Trie`.
    /// Note this leaves the previously allocated capacity.
    pub fn clear(&mut self) {
        self.len = 0;
        self.children.clear();
        self.starts.clear();
    }
    /// Removes from starts vec and removes key, value from children map.
    fn _remove_start(&mut self, key: u128) -> bool {
        if let Some(idx) = self.starts.iter().position(|it| it == &key) {
            self.starts.remove(idx);
            self.children.remove(&key);
            self.len -= 1;
            true
        } else {
            false
        }
    }
    /// `key` is child's key `entry` is child's parent node.
    /// True when node has no children after _remove is called.
    fn _remove(seq: &[T], key: u128, entry: Entry<u128, Node<T>>) -> bool {
        let node = entry
            .and_modify(|n| {
                //println!("{:?}", n);
                n.remove_child(&key);
            })
            // TODO Hacky?? we can't insert on a remove! we know all `keys` in `seq` are valid
            // so if `or_insert_with` runs we have a bug
            .or_insert_with(|| panic!("tried to remove a non existent child {:?}", seq));
        node.child_len() == 0
    }
    /// Returns true if the sequence has been removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ecs_trie::Trie;
    /// let mut trie = Trie::new();
    /// trie.insert(&['c', 'a', 't']);
    /// trie.insert(&['c', 'o', 'w']);
    /// 
    /// assert!(trie.remove(&['c', 'a', 't']));
    /// 
    /// let found = trie.search(&['c']);
    /// assert_eq!(
    ///     found.as_collected().as_slice(),
    ///     &[ ['c', 'o', 'w'] ]
    /// );
    /// ```
    pub fn remove(&mut self, seq: &[T]) -> bool {
        match self.branch_state(seq) {
            Remove::NoMatch => false,
            Remove::Empty => false,
            Remove::Starts(key) => {
                self._remove_start(key)
            },
            Remove::Rest => {
                self.clear();
                true
            },
            Remove::Terminal(mut idx, mut key) => {
                if self.children.remove(&key).is_some() {
                    if let Some(n) = self.children.get_mut(&key_at_index(idx, seq)) {
                        n.remove_child(&key);
                    }
                    self.len -= 1;
                }
                idx += 1;

                while idx < seq.len() {
                    key = key_at_index(idx, seq);
                    if self.children.remove(&key).is_some() {
                        self.len -= 1;
                    }
                    idx += 1;
                }
                
                true
            },
            Remove::Stemish(end_key) => {
                if let Some(node) = self.children.get_mut(&end_key) {
                    node.terminal = false;
                    // self.len -= 1;
                }
                true
            },
            Remove::Childless => {
                let mut i = seq.len() - 1;
                let mut key = key_at_index(i, seq);
                
                while i > 0 {
                    //println!("KE?YAT {:?}", self.children.get(&key_at_index(i - 1, seq)));
                    if Self::_remove(seq, key, self.children.entry(key_at_index(i - 1, seq))) {
                        //println!("KE?YAT {:?}", self.children.get(&key));
                        self.len -= 1;
                        self.children.remove(&key);
                        if i == 1 {
                            let first_key = key_at_index(0, seq);
                            let node = self.children.get(&first_key).expect("key has been checked for match previously bug");
                            if !node.is_terminal() {
                                self._remove_start(first_key);
                                return true;
                            }
                        };
                    } else {
                        // self.len -= 1;
                        return true
                    }
                    i -= 1;
                    key = key_at_index(i, seq);
                }
                true
            },
        }
    }

    fn branch_state(&self, seq: &[T]) -> Remove {
        if self.is_empty() {
            return Remove::Empty;
        }
        if seq.len() == 1 {
            return Remove::Starts(key_from_seq(seq));
        }
        if !seq.iter().enumerate()
            .all(|(i, _)| {
                let key = key_at_index(i, seq);
                self.children.contains_key(&key)
            })
        {
            Remove::NoMatch
        } else if self.len == seq.len() && !self.contains_terminal(seq){
            println!("Rest");
            Remove::Rest
        } else if self.is_terminal_end(seq) {
            println!("Stem");
            let end_key = key_from_seq(seq);
            Remove::Stemish(end_key)
        } else {
            println!("OTHER");
            if let Some((i, non_term_key)) = self.contains_terminal_with_key(seq) {
                println!("seq={:?} idx={} key={}", seq, i, non_term_key);
                return Remove::Terminal(i, non_term_key);
            }
            Remove::Childless
        }
    }
}

// TODO for rev insert
// pub enum Insert {
//     Contains,
//     Child(u64),
//     First(u64),
// }

pub enum Remove {
    /// A parent node has zero or one child and can be removed.
    Childless,
    /// `Trie` is empty.
    Empty,
    /// Removing the last word in the trie short circuts any looping.
    Rest,
    /// Sequence to remove was not found in the `Trie`
    NoMatch,
    /// Single item in sequence, remove from starts.
    Starts(u128),
    /// `Stemish` holds the key to the end node if end node contains children.
    /// The word "car" would be `Stemish` to "cart".
    Stemish(u128),
    /// If sequence contains any terminal nodes, `Terminal` holds the
    /// key to first safe to remove non terminal node.
    Terminal(usize, u128),
}

#[derive(Debug, Clone)]
pub struct Found<T> {
    roll_back: Vec<usize>,
    temp: Vec<T>,
    collected: Vec<Vec<T>>,
}
impl<T: Clone + PartialEq> Found<T> {
    fn new() -> Self {
        Self {
            roll_back: vec![],
            temp: vec![],
            collected: vec![],
        }
    }

    pub fn as_collected(&self) -> Vec<&[T]> {
        self.collected
            .iter()
            .map(|seq| seq.as_slice())
            .collect::<Vec<_>>()
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, i: I) {
        self.temp.extend(i)
    }

    fn push_val(&mut self, t: T) {
        self.temp.push(t);
    }

    fn branch_end_continue(&mut self) {
        self.collected.push(self.temp.clone());
    }

    fn branch_split(&mut self, key: &T) {
        if let Some(idx) = self.temp.iter().position(|item| key == item) {
            let (start, _end) = self.temp.split_at(idx + 1);
            self.temp = start.to_vec();
        }
    }

    fn branch_end(&mut self) {
        self.collected.push(self.temp.clone());
        // remove last element
        self.temp.pop();
    }
}
#[derive(Debug, Clone)]
pub struct TrieIter<'a, T> {
    trie: &'a Trie<T>,
    current: Option<&'a Node<T>>,
    starts: &'a [u128],
    children: Vec<u128>,
    idx: usize,
    next_idx: usize,
}
impl<'a, T> Iterator for TrieIter<'a, T> 
where
    T: Clone + Eq + Hash + Debug,
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
        } else if let Some(key) = self.children.get(self.idx) {
            self.current = self.trie.children.get(&key);
            self.next_idx += 1;

            if self.next_idx >= self.children.len() {
                self.next_idx = 0;
                let curr = self.current.take();
                curr
            } else {
                self.current
            }
        } else {
            let key = self.starts.get(self.idx)?;
            self.current = Some(self.trie.children.get(&key)?);
            self.idx += 1;
            self.current
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    const DATA: &[&str] = &[
        "data/1984.txt",
        "data/sun-rising.txt",
        "data/small.txt",
        "data/words.txt",  
    ];

    fn get_text(i: usize) -> Vec<String> {
        let mut contents = String::new();
        File::open(&DATA[i])
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        contents
            .split_whitespace()
            .map(|s| s.trim().to_lowercase().to_string())
            .collect()
    }

    fn make_trie(words: &[String]) -> Trie<char> {
        let mut trie = Trie::new();
        for w in words {
            trie.insert(&w.to_lowercase().chars().collect::<Vec<_>>());
        }
        trie
    }

    #[test]
    fn insert_find() {
        let cmp_found = vec![ vec!['c', 'a', 't'], vec!['c', 'a', 'r', 't'], vec!['c', 'o', 'w']];
        let mut trie = Trie::new();
        trie.insert(&['c', 'a', 't']);
        trie.insert(&['c', 'a', 'r', 't']);
        trie.insert(&['c', 'o', 'w']);
        let found = trie.search(&['c']);
        // println!("{:?}", found);
        for (expected, found) in cmp_found.iter().zip(found.as_collected()) {
            assert_eq!(&expected[..], found)
        }
    }

    #[test]
    fn trie_iter() {
        let ord = &['c', 'a', 't', 'o', 'w'];

        let mut trie = Trie::new();
        trie.insert(&['c', 'a', 't']);
        trie.insert(&['c', 'o', 'w']);

        for (i, n) in trie.iter().enumerate() {
            assert_eq!(ord[i], n.val)
        }
    }

    #[test]
    fn trie_remove_with_child() {
        let ord = &['c', 'a', 't', 'o', 'w'];

        let mut trie = Trie::new();
        trie.insert(&['c', 'a', 't']);
        trie.insert(&['c', 'a', 'r', 't']);
        trie.insert(&['c', 'o', 'w']);

        trie.remove(&['c', 'a', 'r', 't']);
        for (i, n) in trie.iter().enumerate() {
            assert_eq!(ord[i], n.val)
        }
        trie.remove(&['c', 'o', 'w']);
        trie.remove(&['c', 'a', 't']);
        assert!(trie.is_empty());
    }
    #[test]
    fn trie_remove_with_terminal() {
        let mut t = Trie::new();
        t.insert(&['c', 'a', 'r']);
        t.insert(&['c', 'a', 'r', 't']);
        //t.insert(&['c', 'a', 'r', 't', 'y']);
        t.insert(&['c', 'a', 'r', 'r', 'o', 't']);

        t.remove(&['c', 'a', 'r', 'r', 'o', 't']);
        assert!(t.contains(&['c', 'a', 'r', 't']));

        t.remove(&['c', 'a', 'r', 't']);
        t.remove(&['c', 'a', 'r']);
        println!("{:?}", t);
        assert!(t.is_empty());
    }
    #[test]
    fn trie_remove_with_terminal_end() {
        let mut t = Trie::new();
        t.insert(&['c', 'a', 'r']);
        t.insert(&['c', 'a', 'r', 't']);
        t.insert(&['c', 'a', 'r', 't', 'y']);

        t.remove(&['c', 'a', 'r', 't', 'y']);
        assert!(t.contains(&['c', 'a', 'r', 't']));
        assert!(t.contains(&['c', 'a', 'r']));

        t.remove(&['c', 'a', 'r', 't']);
        t.remove(&['c', 'a', 'r']);
        assert!(t.is_empty());
    }
    #[test]
    fn trie_remove_with_inner_terminal() {
        let mut trie = Trie::new();
        trie.insert(&['c', 'a', 'r']);
        trie.insert(&['c', 'a', 'r', 't']);

        trie.remove(&['c', 'a', 'r']);
        assert!(trie.contains(&['c', 'a', 'r', 't']));
        assert!(!trie.contains(&['c', 'a', 'r']))
    }

    use std::collections::{HashSet, hash_map::RandomState};
    use std::iter::FromIterator;
    #[test]
    fn test_on_data() {
        // test sun rising
        let text = get_text(0);

        let unique: HashSet<_, RandomState> = HashSet::from_iter(text.iter());
        let mut srtd = unique.iter().collect::<Vec<_>>();
        srtd.sort();

        let mut trie = Trie::new();
        for w in srtd.iter() {
            trie.insert(&w.chars().collect::<Vec<_>>());
        }

        for (i, word) in srtd.iter().enumerate() {
            println!("{} at {}/{}  unique: {:?}", word, i + 1, unique.len(), srtd[i]);
            assert!(trie.contains(&word.chars().collect::<Vec<_>>()), "does not contain {}", word);
            trie.remove(&word.chars().collect::<Vec<_>>());
        }

        trie.children.values().for_each(|n| {
            println!("{} {}", n.val, n.child_len())
        });
        println!("{}", trie.len);
        assert!(trie.is_empty());

        // // test 1984
        // let text = get_text(0);
        // let mut trie = make_trie(&text);

        // for word in text.iter() {
        //     assert!(trie.contains(&word.chars().collect::<Vec<_>>()));
        //     trie.remove(&word.chars().collect::<Vec<_>>());
        // }
        // assert!(trie.is_empty());
    }
}
