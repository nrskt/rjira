use std::fs::File;
use std::path::PathBuf;

use backlog::Backlog;
use ports::{BacklogRepository, PortsResult};

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
    async fn get(&self) -> PortsResult<Backlog> {
        let file = File::open(&self.path)?;
        let backlog = serde_yaml::from_reader(file)?;
        Ok(backlog)
    }

    async fn save(&self, backlog: Backlog) -> PortsResult<()> {
        let file = File::open(&self.path)?;
        serde_yaml::to_writer(file, &backlog)?;
        Ok(())
    }
}
