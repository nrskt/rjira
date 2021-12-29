use rest::{
    add_item_handler,
    axum::{
        self,
        routing::{post, put},
        AddExtensionLayer, Router,
    },
    update_item_handler, RestAdaptor,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let adaptors = RestAdaptor::new("./data.yaml");
    let app = Router::new()
        .route("/backlog/items", post(add_item_handler))
        .route("/backlog/items/:item_id", put(update_item_handler))
        .layer(AddExtensionLayer::new(adaptors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
