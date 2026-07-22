## Overview 

`aeonlfu-rs` is my implementation of an LFU (least frequently used) cache in rust.
It uses `SipHash` through the rust `DefaultHasher` as its hashing algorythm and stores
key value pairs up to a certain capacity. When the capacity is reached the next `.insert(key, value)`
will evict the least accessed key value pair. The capacity of the LFU cache is defined by the caller 
in `LfuCache::new(capacity: usize)` and the cache itself is not resizable. The capacity has to be 
a `usize` that is a power of 2.

For more information clone the repository and run
```
cargo doc --open
```
to open the crates documentation in your prefered browser.
This crate was build expirimentally it isn't the best LFU cache but it works and I learned a lot making it.