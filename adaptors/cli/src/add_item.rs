use backlog::{Assignee, BacklogItem, Story, StoryPoint, Task};
use backlog_service::{AddItemCmd, BacklogUseCase, Command, UseCaseResult};

use super::CliAdaptoer;

pub async fn add_item_handler(ctx: CliAdaptoer, cmd: AddItemCliCmd) {
    match ctx.add_item(cmd).await {
        Err(e) => eprintln!("fail: {}", e),
        Ok(backlog) => {
            println!("success");
            println!("{:?}", backlog)
        }
    }
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
    fn item(&self) -> UseCaseResult<Box<dyn backlog::BacklogItem>> {
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
