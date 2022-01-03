use axum::{extract::Extension, Json};
use backlog::{Assignee, Backlog, BacklogItem, Story, StoryPoint, Task};
use backlog_service::{AddItemCmd, BacklogUseCase, Command, IncommingError, IncommingResult};
use serde::Deserialize;

use super::{RestAdaptor, RestError, RestResult};

pub async fn add_item_handler(
    Extension(ctx): Extension<RestAdaptor>,
    Json(payload): Json<AddItemRequest>,
) -> RestResult<Json<Backlog>> {
    ctx.add_item(payload)
        .await
        .map(Json)
        .map_err(RestError::from)
}

#[derive(Deserialize)]
pub struct AddItemRequest {
    item_type: String,
    title: String,
    point: Option<u8>,
    assignee: Option<String>,
}

impl Command for AddItemRequest {}

impl AddItemCmd for AddItemRequest {
    fn item(&self) -> IncommingResult<Box<dyn BacklogItem>> {
        let point = self.point.map(StoryPoint::new).transpose().unwrap();
        let assignee = self.assignee.as_ref().map(|v| Assignee::new(v));
        let item: Box<dyn BacklogItem> = match self.item_type.as_str() {
            "Story" => Box::new(Story::new(&self.title, point, assignee)),
            "Task" => Box::new(Task::new(&self.title, point, assignee)),
            _ => {
                return Err(IncommingError::invalid_value(
                    std::any::type_name::<Box<dyn BacklogItem>>(),
                    format!("item_type does not use the value, {}", self.item_type),
                ))
            }
        };
        Ok(item)
    }
}
