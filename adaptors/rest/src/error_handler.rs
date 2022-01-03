use axum::{http::StatusCode, response::IntoResponse};

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
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)).into_response()
        // let (status, msg) = match &self.0 {
        //     UseCaseError::BacklogRepository(_) => {
        //         (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string())
        //     }
        //     UseCaseError::Backlog(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()),
        //     UseCaseError::NotFound(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()),
        //     UseCaseError::InvalidValue(_) => (StatusCode::BAD_REQUEST, self.0.to_string()),
        // };
        // let body = Json(json!({ "error": msg }));
        // (status, body).into_response()
    }
}
