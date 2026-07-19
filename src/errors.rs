use thiserror::Error;

#[derive(Error, Debug)]
pub enum LfuCacheError {
    #[error("[ERROR]: Capacity {0} is not divisible by two")]
    CapacityNotDivByTwo(usize),
}