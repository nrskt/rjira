use backlog_service::BacklogUseCase;

use super::CliAdaptoer;

pub async fn get_backlog_handler(ctx: CliAdaptoer) {
    match ctx.get_backlog().await {
        Err(e) => eprintln!("fail: {}", e),
        Ok(backlog) => {
            println!("success");
            println!("{:?}", backlog)
        }
    }
}
