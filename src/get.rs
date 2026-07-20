use crate::{LfuCache, Slot};
use crate::hash_key::calc_hash;
use std::hash::Hash;

impl<K, V> LfuCache<K, V> 
    where K: Hash + Clone + Eq, 
    V: Clone + PartialEq {
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
} 