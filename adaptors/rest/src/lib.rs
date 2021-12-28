use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use backlog::{Assignee, Backlog, BacklogItem, Story, StoryPoint, Task};
use backlog_repo::ProvideBacklogRepository;
use backlog_service::{AddItemCmd, BacklogUseCase, Command, UseCaseError};
use fs::FsBacklogRepository;
use serde::Deserialize;
use serde_json::json;

pub async fn add_item(
    Json(payload): Json<AddItemRequest>,
    Extension(ctx): Extension<RestAdaptor>,
) -> Result<Json<Backlog>, RestError> {
    ctx.add_item(payload).await.map(Json).map_err(RestError)
}

#[derive(Debug, Clone)]
pub struct RestAdaptor {
    fs: FsBacklogRepository,
}

impl RestAdaptor {
    pub fn new(path: &str) -> Self {
        Self {
            fs: FsBacklogRepository::new(path.into()),
        }
    }
}

impl BacklogUseCase for RestAdaptor {}

impl ProvideBacklogRepository for RestAdaptor {
    type Repository = FsBacklogRepository;

    fn provide(&self) -> &Self::Repository {
        &self.fs
    }
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
    fn item(&self) -> Box<dyn BacklogItem> {
        let point = self.point.map(StoryPoint::new).transpose().unwrap();
        let assignee = self.assignee.as_ref().map(|v| Assignee::new(v));
        let item: Box<dyn BacklogItem> = match self.item_type.as_str() {
            "Story" => Box::new(Story::new(&self.title, point, assignee)),
            "Task" => Box::new(Task::new(&self.title, point, assignee)),
            _ => todo!(),
        };
        item
    }
}

pub struct RestError(UseCaseError);

impl IntoResponse for RestError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self.0 {
            UseCaseError::Ports(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            UseCaseError::Backlog(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            UseCaseError::NotFound(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
        };
        let body = Json(json!({ "error": msg }));
        (status, body).into_response()
    }
}
