use backlog::{Assignee, BacklogItem, Story, StoryPoint, Task};
use backlog_service::{AddItemCmd, BacklogUseCase, Command, IncommingResult};

use super::{error_handler, CliAdaptoer};

pub async fn add_item_handler(ctx: CliAdaptoer, cmd: AddItemCliCmd) {
    error_handler(|| ctx.add_item(cmd)).await
}

#[derive(Clone, Debug, clap::Parser)]
pub struct AddItemCliCmd {
    item_type: String,
    title: String,
    point: Option<u8>,
    assignee: Option<String>,
}

impl Command for AddItemCliCmd {}

impl AddItemCmd for AddItemCliCmd {
    fn item(&self) -> IncommingResult<Box<dyn BacklogItem>> {
        let point = self.point.map(StoryPoint::new).transpose().unwrap();
        let assignee = self.assignee.as_ref().map(|v| Assignee::new(v));
        let item: Box<dyn BacklogItem> = match self.item_type.as_str() {
            "Story" => Box::new(Story::new(&self.title, point, assignee)),
            "Task" => Box::new(Task::new(&self.title, point, assignee)),
            _ => todo!(),
        };
        Ok(item)
    }
}
