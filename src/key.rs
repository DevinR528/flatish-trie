use std::hash::{Hash, Hasher, BuildHasherDefault};
use std::collections::hash_map::DefaultHasher;

use fasthash::{MetroHasher, FarmHasher, Lookup3Hasher, CityHasher, Murmur3Hasher, SeaHasher};
use fnv::FnvHasher;


/// Length of sequence minus one
pub(crate) fn key_from_seq<T: Hash + Clone>(seq: &[T]) -> Vec<T>  {
    let i = seq.len() - 1;

    seq[..i + 1].to_vec()
}

pub(crate) fn key_at_index<T: Hash + Clone + std::fmt::Debug>(idx: usize, seq: &[T]) -> Vec<T>  {
    println!("{:?}", &seq[..idx + 1]);
    seq[..idx + 1].to_vec()
}
