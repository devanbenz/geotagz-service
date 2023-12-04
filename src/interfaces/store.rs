use std::{error::Error, fmt};

use mockall::automock;

#[derive(Debug, Clone)]
pub enum StoreError {
    StoreInsertError(String),
    StoreGetError(String),
}

#[automock]
pub trait Store<T, R, U> {
    fn insert(&self, data: T) -> Result<(), StoreError>;
    fn find(&self, id: R) -> Result<U, StoreError>;
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::StoreInsertError(msg) => write!(f, "Insert error: {}", msg),
            StoreError::StoreGetError(msg) => write!(f, "Read error: {}", msg),
        }
    }
}

impl Error for StoreError {}
