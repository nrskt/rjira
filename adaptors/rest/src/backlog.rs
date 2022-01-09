use axum::{extract::Extension, Json};
use backlog::Backlog;
use backlog_service::BacklogUseCase;

use super::{RestAdaptor, RestError, RestResult};

#[tracing::instrument]
pub async fn backlog_handler(Extension(ctx): Extension<RestAdaptor>) -> RestResult<Json<Backlog>> {
    ctx.get_backlog().await.map(Json).map_err(RestError::from)
}
