use std::hash::{Hash, Hasher, BuildHasherDefault};
use std::collections::hash_map::DefaultHasher;

use fasthash::{MetroHasher, FarmHasher, Lookup3Hasher, CityHasher, Murmur3Hasher, SeaHasher};
use fnv::FnvHasher;

fn make_key<T: Hash>(to_hash: (&[T], &T), len: usize) -> u64 {
    // let mut hasher = FnvHasher::default();
    // to_hash.hash(&mut hasher);
    // //len.hash(&mut hasher);
    // hasher.finish()
    let mut hasher = Lookup3Hasher::default();
    len.hash(&mut hasher);
    to_hash.hash(&mut hasher);
    hasher.finish()

}
/// Length of sequence minus one
pub(crate) fn key_from_seq<T: Hash>(seq: &[T]) -> u64 {
    let i = seq.len() - 1;
    make_key((&seq[..i], &seq[i]), seq.len())
}

pub(crate) fn key_at_index<T: Hash>(idx: usize, seq: &[T]) -> u64 {
    make_key((&seq[..idx], &seq[idx]), seq.len())
}
