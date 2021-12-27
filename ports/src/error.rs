use std::fmt::Debug;

use thiserror::Error;

pub type PortsResult<T> = Result<T, PortsError>;

#[derive(Debug, Error)]
pub enum PortsError {
    #[error("NotFound: {0:?}")]
    NotFound(String),
}

impl PortsError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}
