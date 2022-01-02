use backlog_service::BacklogUseCase;

use super::{error_handler, CliAdaptoer};

pub async fn get_backlog_handler(ctx: CliAdaptoer) {
    error_handler(|| ctx.get_backlog()).await;
}
