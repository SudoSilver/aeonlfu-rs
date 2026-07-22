//! LfuCacheError is the errors the crate can return.
//! `CapacityNotPowOfTwo` is the error that is returned when the capacity provided in `.new(usize)` is not a power of two.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LfuCacheError {
    #[error("[ERROR]: Capacity {0} is not divisible by two")]
    CapacityNotPowOfTwo(usize),
}