use axum::{http::StatusCode, response::IntoResponse, Json};
use backlog_service::UseCaseError;
use serde_json::json;

pub type RestResult<T> = Result<T, RestError>;

pub struct RestError(UseCaseError);

impl From<UseCaseError> for RestError {
    fn from(err: UseCaseError) -> Self {
        Self(err)
    }
}

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
