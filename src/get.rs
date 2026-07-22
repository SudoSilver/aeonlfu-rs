use crate::{LfuCache, Slot};
use crate::hash_key::calc_hash;
use std::hash::Hash;

impl<K, V> LfuCache<K, V> 
    where K: Hash + Clone + Eq, 
    V: Clone + PartialEq {
    /// `.get(&K)` takes a referance to a key and returns an immutable referance to its value.
    /// If the value does not exist it returns None. 
    /// This incraments the access frequency of the slot which the Key Value pair belongs in.
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
    /// `.get_mut(&K)` takes a referance to a key and returns a mutable referance to its value.
    /// If the value does not exist it returns None.
    /// This incraments the access frequency of the slot which the Key Value pair belongs in.
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
    /// `.contains_key(&K)` returns true if a key exists in the LFU cache otherwise returns false.
    pub fn contains_key(&self, key: &K) -> bool {
        let hash = calc_hash(key);
        let starting_index = hash as usize;
        let mask = self.capacity - 1;
        let mut i: usize = 0;
        loop {
            let index: usize = (starting_index + (i * i + i)/ 2) & mask;

            if let Slot::Occupied(k, _) = &self.data[index] {
                if *k == *key {
                    return true;
                }
                i+=1;
            }else if let Slot::Dead = &self.data[index] {
                i += 1;
            }else if let Slot::NeverOccupied = &self.data[index] {
                return false;
            } 
            if i > mask { return false; }
        }
    }
} 