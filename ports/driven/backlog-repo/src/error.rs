use std::fmt::Debug;

use thiserror::Error;

pub type BacklogRepositoryResult<T> = Result<T, BacklogRepositoryError>;

#[derive(Debug, Error)]
pub enum BacklogRepositoryError {
    #[error("BacklogRepositoryError: not found the resource, {0}")]
    NotFound(String),
    #[error("BacklogRepositoryError: IO occurred something, {0}")]
    Io(#[from] std::io::Error),
    #[error("BacklogRepositoryError: serialize/deserialize yaml occurred something, {0}")]
    Yaml(#[from] serde_yaml::Error),
}

impl BacklogRepositoryError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}
