mod add_item;
mod backlog;
mod error_handler;
mod update_item;

pub use crate::backlog::backlog_handler;
pub use add_item::add_item_handler;
pub use axum;
pub use error_handler::{RestError, RestResult};
pub use update_item::update_item_handler;

use backlog_repo::ProvideBacklogRepository;
use backlog_service::BacklogUseCase;
use fs::FsBacklogRepository;

#[derive(Debug, Clone)]
pub struct RestAdaptor {
    fs: FsBacklogRepository,
}

impl RestAdaptor {
    pub fn new(path: &str) -> Self {
        Self {
            fs: FsBacklogRepository::new(path.into()),
        }
    }
}

impl BacklogUseCase for RestAdaptor {}

impl ProvideBacklogRepository for RestAdaptor {
    type Repository = FsBacklogRepository;

    fn provide(&self) -> &Self::Repository {
        &self.fs
    }
}
