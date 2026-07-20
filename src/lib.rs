pub mod errors;
pub mod hash_key;
mod get;
mod edit;
pub use crate::errors::LfuCacheError;
use std::hash::Hash;

#[derive(Clone)]
pub(crate) enum Slot<K,V> {
    Occupied(K,V),
    Dead,
    NeverOccupied,
}

pub struct LfuCache<K, V> {
    pub(crate) data: Vec<Slot<K,V>>,
    pub(crate) fqs: Vec<Option<u128>>,
    pub(crate) capacity: usize,
    pub(crate) size: usize,
}

impl<K, V> LfuCache<K, V> 
    where K: Hash + Clone + Eq, 
    V: Clone + PartialEq {

    pub fn new(capacity: usize) -> Result<Self, LfuCacheError> {
        if !(capacity % 2 == 0) { 
            return Err(LfuCacheError::CapacityNotDivByTwo(capacity)); 
        }
        return Ok(Self {
            data: vec![Slot::NeverOccupied; capacity],
            fqs: vec![None; capacity],
            capacity,
            size: 0 as usize,
        });
    }
}