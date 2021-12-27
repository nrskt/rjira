use std::fmt::Debug;

use thiserror::Error;

pub type BacklogResult<T> = Result<T, BacklogError>;

#[derive(Debug, Error)]
pub enum BacklogError {
    #[error("TypeError: {0:?}")]
    TypeError(String),
    #[error("NotFound: {0:?}")]
    NotFound(String),
}

impl BacklogError {
    pub fn type_error(msg: impl Into<String>) -> Self {
        Self::TypeError(msg.into())
    }
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}
