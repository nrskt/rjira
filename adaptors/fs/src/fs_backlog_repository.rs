use std::fs::OpenOptions;
use std::path::PathBuf;

use backlog::Backlog;
use backlog_repo::{BacklogRepository, BacklogRepositoryResult};

#[derive(Debug, Clone)]
pub struct FsBacklogRepository {
    path: PathBuf,
}

impl FsBacklogRepository {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[async_trait::async_trait]
impl BacklogRepository for FsBacklogRepository {
    async fn get(&self) -> BacklogRepositoryResult<Backlog> {
        OpenOptions::new()
            .create(true)
            // If I use .write(false), I get the error that mean "InvalidInput".
            .write(true)
            .truncate(false)
            .open(&self.path)?;
        let file = std::fs::File::open(&self.path)?;
        let backlog = serde_yaml::from_reader(file);
        match backlog {
            Err(_) => Ok(Backlog::new()),
            Ok(backlog) => Ok(backlog),
        }
    }

    async fn save(&self, backlog: Backlog) -> BacklogRepositoryResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        serde_yaml::to_writer(file, &backlog)?;
        Ok(())
    }
}
