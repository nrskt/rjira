use backlog::{
    AddItem, AssignableFromCollection, Assignee, Backlog, BacklogItem, EstimatableFromCollection,
    StoryPoint, Uuid,
};
use backlog_repo::{BacklogRepository, ProvideBacklogRepository};

use crate::UseCaseResult;

#[async_trait::async_trait]
pub trait BacklogUseCase: ProvideBacklogRepository {
    /// Add item to backlog
    async fn add_item(&self, cmd: impl AddItemCmd + 'async_trait) -> UseCaseResult<Backlog> {
        let repo = self.provide();
        let mut backlog = repo.get().await?;
        backlog.add_item(cmd.item());
        repo.save(backlog.clone()).await?;
        Ok(backlog)
    }

    /// Assign the specific item to someone.
    async fn assign_item(&self, cmd: impl AssignItemCmd + 'async_trait) -> UseCaseResult<Backlog> {
        let repo = self.provide();
        let mut backlog = repo.get().await?;
        backlog.assign_item(&cmd.id(), cmd.assignee())?;
        repo.save(backlog.clone()).await?;
        Ok(backlog)
    }

    /// Estimate the specific item.
    async fn estimate_item(
        &self,
        cmd: impl EstimateItemCmd + 'async_trait,
    ) -> UseCaseResult<Backlog> {
        let repo = self.provide();
        let mut backlog = repo.get().await?;
        backlog.estimate_item(&cmd.id(), cmd.point())?;
        repo.save(backlog.clone()).await?;
        Ok(backlog)
    }
}

pub trait Command: Send {}

pub trait AddItemCmd: Command {
    fn item(&self) -> Box<dyn BacklogItem>;
}

pub trait AssignItemCmd: Command {
    fn id(&self) -> Uuid;
    fn assignee(&self) -> Assignee;
}

pub trait EstimateItemCmd: Command {
    fn id(&self) -> Uuid;
    fn point(&self) -> StoryPoint;
}
