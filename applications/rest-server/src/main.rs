use axum::{routing::post, AddExtensionLayer, Router};
use rest::{add_item, RestAdaptor};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let adaptors = RestAdaptor::new("./data.json");
    let app = Router::new()
        .route("/backlog/item", post(add_item))
        .layer(AddExtensionLayer::new(adaptors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
