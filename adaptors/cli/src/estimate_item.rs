use backlog::{StoryPoint, Uuid};
use backlog_service::{
    BacklogUseCase, Command, EstimateItemCmd, IncommingError, IncommingResult, UseCaseResult,
};

use super::{error_handler, CliAdaptoer};

pub async fn estimate_item_handler(ctx: CliAdaptoer, cmd: EstimateItemCliCmd) {
    error_handler(|| ctx.estimate_item(cmd)).await
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
    fn point(&self) -> IncommingResult<StoryPoint> {
        StoryPoint::new(self.point).map_err(|err| IncommingError::invalid_value("invalid", err.to_string()))
    }
}
