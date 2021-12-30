use axum::{
    extract::{Extension, Path},
    Json,
};
use backlog::{Assignee, Backlog, StoryPoint, Uuid};
use backlog_service::{
    AssignItemCmd, BacklogUseCase, Command, EstimateItemCmd, UseCaseError, UseCaseResult,
};
use serde::Deserialize;

use super::{RestAdaptor, RestError, RestResult};

pub async fn update_item_handler(
    Extension(ctx): Extension<RestAdaptor>,
    Path(item_id): Path<Uuid>,
    Json(payload): Json<UpdateItemRequest>,
) -> RestResult<Json<Backlog>> {
    if let Some(point) = payload.point {
        let req = EstimateRequest { id: item_id, point };
        ctx.estimate_item(req)
            .await
            .map(Json)
            .map_err(RestError::from)?;
    }

    if let Some(assignee) = payload.assignee {
        let req = AssignRequest {
            id: item_id,
            assignee,
        };
        ctx.assign_item(req)
            .await
            .map(Json)
            .map_err(RestError::from)?;
    }
    ctx.get_backlog().await.map(Json).map_err(RestError::from)
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    point: Option<u8>,
    assignee: Option<String>,
}

struct EstimateRequest {
    id: Uuid,
    point: u8,
}

impl Command for EstimateRequest {}

impl EstimateItemCmd for EstimateRequest {
    fn id(&self) -> UseCaseResult<Uuid> {
        Ok(self.id)
    }

    fn point(&self) -> UseCaseResult<StoryPoint> {
        println!("error_handling test");
        Ok(StoryPoint::new(self.point)
            .map_err(|err| UseCaseError::invalid_value(err.to_string()))?)
    }
}

struct AssignRequest {
    id: Uuid,
    assignee: String,
}

impl Command for AssignRequest {}

impl AssignItemCmd for AssignRequest {
    fn id(&self) -> UseCaseResult<Uuid> {
        Ok(self.id)
    }

    fn assignee(&self) -> UseCaseResult<Assignee> {
        Ok(Assignee::new(&self.assignee))
    }
}
