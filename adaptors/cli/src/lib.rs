mod add_item;
mod assign_item;
mod backlog;
mod error_handler;
mod estimate_item;

pub use clap;
pub use error_handler::error_handler;

use std::path::PathBuf;

use crate::backlog::get_backlog_handler;
use add_item::{add_item_handler, AddItemCliCmd};
use assign_item::{assign_item_handler, AssignItemCliCmd};
use backlog_repo::ProvideBacklogRepository;
use backlog_service::BacklogUseCase;
use clap::Parser;
use estimate_item::{estimate_item_handler, EstimateItemCliCmd};
use fs::FsBacklogRepository;

pub struct CliAdaptoer {
    fs: FsBacklogRepository,
}

impl CliAdaptoer {
    pub fn new(path: PathBuf) -> Self {
        Self {
            fs: FsBacklogRepository::new(path),
        }
    }
}

impl BacklogUseCase for CliAdaptoer {}

impl ProvideBacklogRepository for CliAdaptoer {
    type Repository = FsBacklogRepository;

    fn provide(&self) -> &Self::Repository {
        &self.fs
    }
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    command: SubCommand,
    #[clap(short, default_value = "data.yaml")]
    data: std::path::PathBuf,
}

impl Args {
    pub fn data(&self) -> PathBuf {
        self.data.clone()
    }

    pub async fn run(&self, adaptors: CliAdaptoer) {
        match &self.command {
            SubCommand::GetBacklog => get_backlog_handler(adaptors).await,
            SubCommand::AddItem(cmd) => add_item_handler(adaptors, cmd.clone()).await,
            SubCommand::EstimateItem(cmd) => estimate_item_handler(adaptors, cmd.clone()).await,
            SubCommand::AssignItem(cmd) => assign_item_handler(adaptors, cmd.clone()).await,
        }
    }
}
#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    GetBacklog,
    AddItem(AddItemCliCmd),
    EstimateItem(EstimateItemCliCmd),
    AssignItem(AssignItemCliCmd),
}
