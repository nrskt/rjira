use backlog::Backlog;

use crate::BacklogRepositoryResult;

pub trait ProvideBacklogRepository {
    type Repository: BacklogRepository + Send + Sync;

    fn provide(&self) -> &Self::Repository;
}

#[async_trait::async_trait]
pub trait BacklogRepository {
    /// Get the specific backlog.
    ///
    /// If backlog does not find, return the error.
    async fn get(&self) -> BacklogRepositoryResult<Backlog>;

    /// Save the specific backlog.
    async fn save(&self, backlog: Backlog) -> BacklogRepositoryResult<()>;
}
