use std::fmt::Debug;

use backlog::BacklogError;
use backlog_repo::BacklogRepositoryError;
use thiserror::Error;

pub type UseCaseResult<T> = Result<T, UseCaseError>;

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("Domain BacklogError: {0}")]
    Backlog(#[from] BacklogError),
    #[error("UseCaseError: {0}")]
    BacklogRepository(#[from] BacklogRepositoryError),
    #[error("NotFound: {0:?}")]
    NotFound(String),
    #[error("InvalidValue: {0}")]
    InvalidValue(String),
}

impl UseCaseError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn invalid_value(msg: impl Into<String>) -> Self {
        Self::InvalidValue(msg.into())
    }
}
