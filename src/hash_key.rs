use std::hash::Hash;
use std::hash::DefaultHasher;
use std::hash::Hasher;

pub fn calc_hash<K>(key: &K) -> u64 where for<'a> &'a K: Eq + Clone + Hash {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    
    return hasher.finish();
}