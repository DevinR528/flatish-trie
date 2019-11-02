use std::hash::{Hash, Hasher, BuildHasherDefault};

use fnv::FnvHasher;

pub(crate) fn make_key<T: Hash>(to_hash: (&[T], &T)) -> u64 {
    let mut hasher = FnvHasher::default();
    to_hash.hash(&mut hasher);
    hasher.finish()
}

pub(crate) fn key_from_seq<T: Hash>(seq: &[T]) -> u64 {
    let i = seq.len() - 1;
    make_key((&seq[..i], &seq[i]))
}

pub(crate) fn key_at_index<T: Hash>(idx: usize, seq: &[T]) -> u64 {
    make_key((&seq[..idx], &seq[idx]))
}
