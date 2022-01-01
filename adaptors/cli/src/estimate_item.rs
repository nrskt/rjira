use backlog::{StoryPoint, Uuid};
use backlog_service::{BacklogUseCase, Command, EstimateItemCmd, UseCaseError, UseCaseResult};

use super::CliAdaptoer;

pub async fn estimate_item_handler(ctx: CliAdaptoer, cmd: EstimateItemCliCmd) {
    match ctx.estimate_item(cmd).await {
        Err(e) => eprintln!("fail: {}", e),
        Ok(backlog) => {
            println!("success");
            println!("{:?}", backlog)
        }
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub struct EstimateItemCliCmd {
    id: Uuid,
    point: u8,
}

impl Command for EstimateItemCliCmd {}

impl EstimateItemCmd for EstimateItemCliCmd {
    fn id(&self) -> UseCaseResult<Uuid> {
        Ok(self.id)
    }
    fn point(&self) -> UseCaseResult<StoryPoint> {
        StoryPoint::new(self.point).map_err(|err| UseCaseError::invalid_value(err.to_string()))
    }
}
