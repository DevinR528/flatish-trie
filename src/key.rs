use std::hash::{Hash, Hasher, BuildHasherDefault};
use std::collections::hash_map::DefaultHasher;

use fasthash::{HasherExt, SpookyHasherExt, MetroHasherExt, FarmHasher, Lookup3Hasher, CityHasher, Murmur3HasherExt, SeaHasher};
use fnv::FnvHasher;

fn make_key<T: Hash>(to_hash: (&[T], &T), len: usize, seq: &[T]) -> u128 {
    // let mut hasher = FnvHasher::default();
    // to_hash.hash(&mut hasher);
    // //len.hash(&mut hasher);
    // hasher.finish()
    let mut hasher = MetroHasherExt::default();
    //len.hash(&mut hasher);
    to_hash.hash(&mut hasher);
    //seq.hash(&mut hasher);
    hasher.finish_ext()

}
/// Length of sequence minus one
pub(crate) fn key_from_seq<T: Hash>(seq: &[T]) -> u128 {
    let i = seq.len() - 1;
    make_key((&seq[..i], &seq[i]), seq.len(), seq)
}
/// End of sequence
pub(crate) fn key_at_index<T: Hash>(idx: usize, seq: &[T]) -> u128 {
    make_key((&seq[..idx], &seq[idx]), seq.len(), seq)
}
