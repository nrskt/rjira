use axum::{http::StatusCode, response::IntoResponse, Json};
use backlog_service::{BusinessLogicError, IncommingError, OutcommingError};
use serde_json::json;

pub type RestResult<T> = Result<T, RestError>;

#[derive(Debug)]
pub struct RestError(eyre::Error);

impl From<eyre::Error> for RestError {
    fn from(err: eyre::Error) -> Self {
        Self(err)
    }
}

impl IntoResponse for RestError {
    #[tracing::instrument]
    fn into_response(self) -> axum::response::Response {
        let RestError(err) = self;

        let (status, msg) = if err.downcast_ref::<IncommingError>().is_some() {
            tracing::error!("BAD REQUEST: {:?}", err);
            (StatusCode::BAD_REQUEST, format!("{:?}", err))
        } else if err.downcast_ref::<OutcommingError>().is_some() {
            tracing::error!("INTERNAL_SERVER_ERROR: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
        } else if err.downcast_ref::<BusinessLogicError>().is_some() {
            tracing::error!("INTERNAL_SERVER_ERROR: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
        } else {
            tracing::error!("unexpected error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unexpected error".to_string(),
            )
        };

        let body = Json(json!({ "error": msg }));
        (status, body).into_response()
    }
}
