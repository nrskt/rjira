use backlog::{Assignee, Uuid};
use backlog_service::{AssignItemCmd, BacklogUseCase, Command};

use super::CliAdaptoer;

pub async fn assign_item_handler(ctx: CliAdaptoer, cmd: AssignItemCliCmd) {
    match ctx.assign_item(cmd).await {
        Err(e) => eprintln!("fail: {}", e),
        Ok(backlog) => {
            println!("success");
            println!("{:?}", backlog)
        }
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub struct AssignItemCliCmd {
    id: Uuid,
    assignee: String,
}

impl Command for AssignItemCliCmd {}

impl AssignItemCmd for AssignItemCliCmd {
    fn id(&self) -> Uuid {
        self.id
    }
    fn assignee(&self) -> Assignee {
        Assignee::new(&self.assignee)
    }
}
