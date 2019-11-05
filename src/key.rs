use std::hash::{Hash, Hasher, BuildHasherDefault};
use std::collections::hash_map::DefaultHasher;

use fasthash::{HasherExt, SpookyHasherExt, MetroHasherExt, FarmHasher, Lookup3Hasher, CityHasherExt, Murmur3HasherExt, SeaHasher};
use fnv::FnvHasher;

fn make_key<T: Hash>(to_hash: (&[T], &T)) -> u128 {
    let mut hasher = CityHasherExt::default();
    to_hash.hash(&mut hasher);
    hasher.finish_ext()
}
/// Length of sequence minus one
pub(crate) fn key_from_seq<T: Hash>(seq: &[T]) -> u128 {
    let i = seq.len() - 1;
    make_key((&seq[..i], &seq[i]))
}
/// End of sequence
pub(crate) fn key_at_index<T: Hash>(idx: usize, seq: &[T]) -> u128 {
    make_key((&seq[..idx], &seq[idx]))
}
