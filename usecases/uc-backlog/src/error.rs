use std::fmt::Debug;

use backlog::BacklogError;
use ports::PortsError;
use thiserror::Error;

pub type UseCaseResult<T> = Result<T, UseCaseError>;

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("Domain BacklogError: {0}")]
    Backlog(#[from] BacklogError),
    #[error("PortsError: {0}")]
    Ports(#[from] PortsError),
    #[error("NotFound: {0:?}")]
    NotFound(String),
}

impl UseCaseError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}
