//! aeonlfu-rs is a simple LFU cache (least frequently used) implemented in a simple crate with a simple API
//! The crate hashes the keys with SipHash through rusts DefaultHasher property.
//! It also utilizes an enum of Slot<K,V> with the variants Occupied(K,V), Dead and NeverOccupied.
//! The keys of type K must be unique accross every entry and implement Hash, Clone and Eq impls from the rust standard library.
//! Values are of type V and need to implement Clone and PartialEq impls from the rust standard library.
//! All keys and values must be of the same type. 
//! Operations that have access to the value incrament the access frequency of the field where as only accessing the key does not.
pub mod errors;
mod hash_key;
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

/// LfuCache<K,V> is the struct that the LFU cache uses to store values and apply methods on.
pub struct LfuCache<K, V> {
    pub(crate) data: Vec<Slot<K,V>>,
    pub(crate) fqs: Vec<Option<u128>>,
    pub(crate) capacity: usize,
    pub(crate) size: usize,
}

impl<K, V> LfuCache<K, V> 
    where K: Hash + Clone + Eq, 
    V: Clone + PartialEq {
    /// The `::new(usize)` property creates the LfuCache struct with the specified capacity. 
    pub fn new(capacity: usize) -> Result<Self, LfuCacheError> {
        if capacity == 0 || (capacity & (capacity - 1)) != 0 {  
            return Err(LfuCacheError::CapacityNotPowOfTwo(capacity)); 
        }
        return Ok(Self {
            data: vec![Slot::NeverOccupied; capacity],
            fqs: vec![None; capacity],
            capacity,
            size: 0 as usize,
        });
    }
    /// `.len()` returns the amount of used slots as a usize 
    pub fn len(&self) -> usize {
        return self.size;
    }
    /// `.is_empty()` checks if all slots currently dont hold a value.
    pub fn is_empty(&self) -> bool {
        if self.size == 0 {
            return true;
        }
        return false;
    }
}