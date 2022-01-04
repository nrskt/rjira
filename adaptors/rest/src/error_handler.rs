use axum::{http::StatusCode, response::IntoResponse, Json};
use backlog_service::{BusinessLogicError, IncommingError, OutcommingError};
use serde_json::json;

pub type RestResult<T> = Result<T, RestError>;

pub struct RestError(eyre::Error);

impl From<eyre::Error> for RestError {
    fn from(err: eyre::Error) -> Self {
        Self(err)
    }
}

impl IntoResponse for RestError {
    fn into_response(self) -> axum::response::Response {
        let RestError(err) = self;

        let (status, msg) = if let Some(_) = err.downcast_ref::<IncommingError>() {
            (StatusCode::BAD_REQUEST, format!("{:?}", err))
        } else if let Some(_) = err.downcast_ref::<OutcommingError>() {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
        } else if let Some(_) = err.downcast_ref::<BusinessLogicError>() {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("unexpected error"),
            )
        };

        let body = Json(json!({ "error": msg }));
        (status, body).into_response()
    }
}
