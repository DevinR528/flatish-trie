use std::hash::{Hash, Hasher, BuildHasherDefault};
use std::collections::hash_map::DefaultHasher;

use fasthash::{MetroHasher, FarmHasher, Lookup3Hasher, CityHasher, Murmur3Hasher, SeaHasher};
use fnv::FnvHasher;

fn make_key<T>(to_hash: (&[T], &T), len: usize) -> (u64, T) 
where
    T: Clone + Hash,
{
    let mut hasher = SeaHasher::default();
    to_hash.hash(&mut hasher);
    (hasher.finish(), to_hash.1.clone())
}
/// Length of sequence minus one
pub(crate) fn key_from_seq<T: Hash + Clone>(seq: &[T]) -> (u64, T)  {
    let i = seq.len() - 1;
    make_key((&seq[..i], &seq[i]), seq.len())
}

pub(crate) fn key_at_index<T: Hash + Clone>(idx: usize, seq: &[T]) -> (u64, T)  {
    make_key((&seq[..idx], &seq[idx]), seq.len())
}
