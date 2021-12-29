use backlog::{
    AddItem, AssignableFromCollection, Assignee, Backlog, BacklogItem, EstimatableFromCollection,
    StoryPoint, Uuid,
};
use backlog_repo::{BacklogRepository, ProvideBacklogRepository};

use crate::UseCaseResult;

#[async_trait::async_trait]
pub trait BacklogUseCase: ProvideBacklogRepository {
    async fn get_backlog(&self) -> UseCaseResult<Backlog> {
        let repo = self.provide();
        let backlog = repo.get().await?;
        Ok(backlog)
    }

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

#[cfg(test)]
mod test_get_backlog {
    use super::*;

    #[tokio::test]
    async fn test_get_backlog() {
        let mut mock = mock::MockTest::new();
        mock.expect_get().times(1).returning(|| Ok(Backlog::new()));
        mock.get_backlog().await.unwrap();
    }
}

#[cfg(test)]
mod test_add_item {
    use super::*;
    use backlog::{FindFromCollection, Story};

    #[tokio::test]
    async fn test_add_item() {
        let mut mock = mock::MockTest::new();
        mock.expect_get().times(1).returning(|| Ok(Backlog::new()));
        mock.expect_save()
            .times(1)
            .withf(|backlog| backlog.len() == 1)
            .returning(|_| Ok(()));

        let mut cmd = mock::MockAddItemCmd::new();
        cmd.expect_item().returning(|| {
            let story = Story::new("", None, None);
            Box::new(story)
        });
        mock.add_item(cmd).await.unwrap();
    }
}

#[cfg(test)]
mod test_estimate_item {
    use super::*;
    use backlog::BacklogFixture;
    use serde_json::json;

    #[tokio::test]
    async fn test_estimate_item() {
        let (item_id, backlog) = Backlog::specific_id();

        let mut mock = mock::MockTest::new();
        mock.expect_get()
            .times(1)
            .returning(move || Ok(backlog.clone()));
        mock.expect_save()
            .times(1)
            .withf(|backlog| {
                let finder = mock::finder(
                    backlog,
                    "$.items.ec1985c0-b7ee-4556-a0d1-461ee9eb754f.point",
                );
                finder.find() == json!([1])
            })
            .returning(|_| Ok(()));

        let mut cmd = mock::MockEstimateItemCmd::new();
        cmd.expect_id().returning(move || item_id);
        cmd.expect_point().returning(|| StoryPoint::new(1).unwrap());

        assert!(mock.estimate_item(cmd).await.is_ok());
    }
}

#[cfg(test)]
mod test_assign_item {
    use super::*;
    use backlog::BacklogFixture;
    use serde_json::json;

    #[tokio::test]
    async fn test_assign_item() {
        let (item_id, backlog) = Backlog::specific_id();

        let mut mock = mock::MockTest::new();
        mock.expect_get()
            .times(1)
            .returning(move || Ok(backlog.clone()));
        mock.expect_save()
            .times(1)
            .withf(|backlog| {
                let finder = mock::finder(
                    backlog,
                    "$.items.ec1985c0-b7ee-4556-a0d1-461ee9eb754f.assignee",
                );
                finder.find() == json!(["dummy"])
            })
            .returning(|_| Ok(()));

        let mut cmd = mock::MockAssignItemCmd::new();
        cmd.expect_id().returning(move || item_id);
        cmd.expect_assignee().returning(|| Assignee::new("dummy"));

        assert!(mock.assign_item(cmd).await.is_ok());
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use async_trait::async_trait;
    use backlog_repo::PortsResult;
    use jsonpath_rust::*;
    use mockall::mock;

    pub fn finder(value: impl serde::Serialize, path: &str) -> JsonPathFinder {
        let ser = serde_json::to_string(&value).expect("fail to serialize");
        JsonPathFinder::from_str(&ser, path).expect("fail to create JsonPathFinder")
    }

    mock! {
        pub Test {}

        #[async_trait]
        impl BacklogRepository for Test {
            async fn get(&self) -> PortsResult<Backlog>;
            async fn save(&self, backlog: Backlog) -> PortsResult<()>;
        }
    }

    mock! {
        pub AddItemCmd {}

        impl Command for AddItemCmd {}

        impl AddItemCmd for AddItemCmd {
            fn item(&self) -> Box<dyn BacklogItem>;
        }
    }

    mock! {
        pub EstimateItemCmd {}

        impl Command for EstimateItemCmd {}

        impl EstimateItemCmd for EstimateItemCmd {
            fn id(&self) -> Uuid;
            fn point(&self) -> StoryPoint;
        }
    }

    mock! {
        pub AssignItemCmd {}

        impl Command for AssignItemCmd {}

        impl AssignItemCmd for AssignItemCmd {
            fn id(&self) -> Uuid;
            fn assignee(&self) -> Assignee;
        }
    }

    impl BacklogUseCase for MockTest {}

    impl ProvideBacklogRepository for MockTest {
        type Repository = MockTest;

        fn provide(&self) -> &Self::Repository {
            &self
        }
    }
}
