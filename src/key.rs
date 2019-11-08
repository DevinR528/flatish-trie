use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

use fasthash::{CityHasher, FarmHasher, Lookup3Hasher, MetroHasher, Murmur3Hasher, SeaHasher};
use fnv::FnvHasher;

/// Length of sequence minus one
pub(crate) fn key_from_seq<T: Hash + Clone>(seq: &[T]) -> Vec<T> {
    seq[..seq.len()].to_vec()
}

pub(crate) fn key_at_index<T: Hash + Clone + std::fmt::Debug>(idx: usize, seq: &[T]) -> Vec<T> {
    seq[..idx + 1].to_vec()
}
