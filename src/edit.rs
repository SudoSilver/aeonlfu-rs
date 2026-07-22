use crate::{LfuCache, Slot};
use crate::hash_key::calc_hash;
use std::hash::Hash;

impl<K, V> LfuCache<K, V> 
    where K: Hash + Clone + Eq, 
    V: Clone + PartialEq {
    /// `.insert(K,V)` inserts a key value pair to the LFU cache after hashing its key. 
    /// In case the LFU cache is full it calls `.remove_least_used()` to clean up the least used slot.
    pub fn insert(&mut self, key: K, value: V) {
        let hash = calc_hash(&key);
        let starting_index = hash as usize;
        let mask = self.capacity - 1;
        let mut i: usize = 0;

        if self.capacity == self.size {
            self.remove_least_used();
        }

        loop {
            let index: usize = (starting_index + (i * i + i) / 2) & mask;

            if let Slot::Occupied(k, _) = &self.data[index] {
                if *k == key {
                    self.data[index] = Slot::Occupied(key.clone(), value.clone());
                    let Some(ref mut fq) = self.fqs[index] else { return; };
                    *fq += 1;
                    return;
                }else {
                    i+=1;
                }
            }else {
                self.data[index] = Slot::Occupied(key.clone(), value.clone());
                self.fqs[index] = Some(0 as u128);
                self.size += 1;
                return;
            }
            if i == mask { return; }
        }
    }
    /// `.remove(K)` removes a single key and returns its value in case the key does not exist it returns None.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let hash = calc_hash(&key);
        let starting_index = hash as usize;
        let mask = self.capacity - 1;
        let mut i: usize = 0;
        
        loop {
            let index: usize = (starting_index + (i * i + i) / 2) & mask;
            if let Slot::Occupied(k, v) = &self.data[index] {
                if *k == *key {
                    let value = v.clone();
                    self.fqs[index] = None;
                    self.data[index] = Slot::Dead;
                    self.size -= 1;
                    return Some(value);
                }else {
                    i+=1;
                }
            }else if let Slot::NeverOccupied = &self.data[index] {
                return None;
            }else {
                i+=1;
            }
            if i == mask { return None; }
        }
    }
    /// `.remove_least_used()` cleans up the least used slot.
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