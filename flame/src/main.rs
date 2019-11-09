use std::collections::{hash_map::RandomState, HashSet};
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

// extern crate flame;
// #[macro_use] extern crate flamer;

use ecs_trie::Trie;

const DATA: &[&str] = &["../data/1984.txt", "../data/sun-rising.txt", "data/words.txt"];

fn get_text(i: usize) -> Vec<String> {
    let mut contents = String::new();
    File::open(&DATA[i])
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
        .split_whitespace()
        .map(|s| s.trim().to_string())
        .collect()
}
// #[flame]
fn make_trie(words: &[String]) -> Trie<char> {
    let mut trie = Trie::new();
    for w in words {
        trie.insert(&w.chars().collect::<Vec<_>>());
    }
    
    trie
}
// #[flame]
fn trie_insert() {
    let words = get_text(1);
    make_trie(&words);
}

fn trie_get() {
    let words = get_text(1);
    let trie = make_trie(&words);
    
    for w in words.iter() {
        trie.search(&w.chars().collect::<Vec<_>>());
    }
}

fn trie_insert_remove() {
    

    
}

fn main() {
    // trie_insert();
    // trie_get();

    for _ in 0..1000 {
        let words = get_text(0);
        let mut trie = make_trie(&words);
        // for w in &words {
        //     trie.remove(&w.chars().collect::<Vec<_>>());
        // }
    }

    //::flame::dump_html(File::create("trie_insert_remove.html").unwrap()).unwrap();
}
