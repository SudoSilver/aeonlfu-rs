use aeonlfu_rs::*;

#[test]
fn smoke_test() {
    let mut cache: LfuCache<String, u64> = match LfuCache::new(4 as usize) {
        Ok(c) => c,
        Err(_) => panic!("[ERROR]: Something went horribly wrong with the LFU cache generation"),
    };
    assert!(cache.is_empty(), "[TEST]: The is_empty check returns false on an empty cache");

    cache.insert("Hello".to_string(), 5 as u64);
    let get_result = cache.get(&"Hello".to_string());
    assert_eq!(get_result, Some(5).as_ref(), "[TEST]: get does not return the correct result");

    cache.insert("Hello".to_string(), 10 as u64);
    let get_result2 = cache.get(&"Hello".to_string());
    assert_eq!(get_result2, Some(10).as_ref(), "[TEST]: Insert on an already existing key does not update it");

    cache.insert("World".to_string(), 10 as u64);
    if let Some(value) = cache.get_mut(&"World".to_string()) {
        *value += 1;
    }
    if let Some(value) = cache.get(&"World".to_string()) {
        assert_eq!(value, &(11 as u64));
    }

    cache.insert("Day".to_string(), 10 as u64);
    cache.insert("Night".to_string(), 10 as u64);

    assert_eq!(cache.len(), 4);
    assert!(cache.contains_key(&"Night".to_string()), "[TEST]: Cant find key with contains_key");
    assert!(!(cache.is_empty()), "[TEST]: The is_empty check returns true on a non empty cache");

    let _ = cache.get(&"Night".to_string());    

    cache.remove_least_used();
    assert_ne!(cache.contains_key(&"Day".to_string()), true, "[TEST]: Least used key was not evicted");

    let v = cache.remove(&"Hello".to_string());
    assert_eq!(v, Some(10 as u64), "[TEST]: Value was not returned after removing a key");
    assert!(!(cache.contains_key(&"Hello".to_string())), "[TEST]: Key was not actually removed");
    assert_eq!(cache.len(), 2, "[TEST]: Length was no decremented properly");
    assert!(cache.remove(&"Hello".to_string()).is_none(), "[TEST]: Removal of a non existant key does not return None");
}