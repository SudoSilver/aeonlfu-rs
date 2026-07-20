pub mod errors;
pub mod hash_key;
pub use crate::errors::LfuCacheError;
use crate::hash_key::calc_hash;
use std::hash::Hash;

#[derive(Clone)]
enum Slot<K,V> {
    Occupied(K,V),
    Dead,
    NeverOccupied,
}

pub struct LfuCache<K, V> {
    data: Vec<Slot<K,V>>,
    fqs: Vec<Option<u128>>,
    capacity: usize,
    size: usize,
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

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let hash = calc_hash(&key);
        let starting_index = hash as usize;
        let mask = self.capacity - 1;
        let mut i: usize = 0;
        loop {
            let index: usize = (starting_index + (i * i + i) / 2) & mask;

            if let Slot::Occupied(k,v) = &self.data[index] {
                if *k == *key && let Some(ref mut fq) = self.fqs[index]{
                    *fq += 1; 
                    return Some(v);
                }else{
                    i+=1;
                }
            }else if let Slot::NeverOccupied = &self.data[index]{
                break;
            }else {
                i+=1;
            }

            if i == mask { break; }
        }
        return None;
    } 
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let hash = calc_hash(&key);
        let starting_index = hash as usize;
        let mask = self.capacity - 1;
        let mut i: usize = 0;
        loop {
            let index: usize = (starting_index + (i * i + i) / 2) & mask;

            let matched = match &self.data[index] {
                Slot::Occupied(k, _) => *k == *key,
                _ => false,
            };

            if matched {
                if let Some(ref mut fq) = self.fqs[index] {
                    *fq += 1;
                }
                return match &mut self.data[index] {
                    Slot::Occupied(_, v) => Some(v),
                    _ => None,
                };
            }

            let is_never_occupied = match &self.data[index] {
                Slot::NeverOccupied => true,
                _ => false,
            };
            if is_never_occupied {
                break;
            }

            i += 1;
            if i == mask { break; }
        }
        return None;
    } 
    pub fn remove_least_used(&mut self) {
        let mut i: usize = 0;
        let mut least_used: Option<usize> = None;
        let mut lowest_usage: u128 = u128::MAX;  

        while i < self.capacity {
            if let Some(fq) = self.fqs[i] {
                if fq < lowest_usage {
                    least_used = Some(i);
                    lowest_usage = fq;
                }
            }  
            i += 1;
        } 
 
        if let Some(index) = least_used {
            self.fqs[index] = None;
            self.data[index] = Slot::Dead;
            self.size -= 1;
        }
    }
}