use std::fmt::Debug;

use thiserror::Error;

pub type PortsResult<T> = Result<T, PortsError>;

#[derive(Debug, Error)]
pub enum PortsError {
    #[error("NotFound: {0:?}")]
    NotFound(String),
    #[error("InternalError: {0}")]
    InternalError(String),
}

impl PortsError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}

impl From<serde_yaml::Error> for PortsError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}

impl From<std::io::Error> for PortsError {
    fn from(err: std::io::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}
