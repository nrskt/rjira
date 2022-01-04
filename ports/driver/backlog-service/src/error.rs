use std::fmt::Debug;

use backlog::BacklogError;
use backlog_repo::BacklogRepositoryError;
use thiserror::Error;

pub type IncommingResult<T> = Result<T, IncommingError>;

#[derive(Debug, Error)]
pub enum IncommingError {
    #[error("InvalidValue: type,{resource}, {msg}")]
    InvalidValue { resource: TypeName, msg: String },
}

impl IncommingError {
    pub fn invalid_value(resource: TypeName, msg: impl Into<String>) -> Self {
        Self::InvalidValue {
            resource,
            msg: msg.into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum OutcommingError {
    #[error("BacklogRepositoryError: {0}")]
    BacklogRepository(#[from] BacklogRepositoryError),
}

#[derive(Debug, Error)]
pub enum BusinessLogicError {
    #[error("Domain BacklogError: {0}")]
    Backlog(#[from] BacklogError),
    #[error("NotFound: resource, {resource} is not found. detail, {msg}")]
    NotFound { resource: TypeName, msg: String },
}

type TypeName = &'static str;
