use std::net::SocketAddr;

use hyper::{header::HeaderValue, http::Request, Body};
use rest::{
    add_item_handler,
    axum::{
        self,
        routing::{get, post, put},
        AddExtensionLayer, Router,
    },
    backlog_handler, update_item_handler, RestAdaptor,
};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{self, fmt};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .json()
                .with_span_list(true)
                .with_current_span(false),
        )
        .fmt_fields(fmt::format::JsonFields::default())
        .init();
    let adaptors = RestAdaptor::new("./data.yaml");
    let app = Router::new()
        .route("/backlog", get(backlog_handler))
        .route("/backlog/items", post(add_item_handler))
        .route("/backlog/items/:item_id", put(update_item_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request<Body>| {
                    let mut headers = req.headers().clone();
                    headers.insert("authorization", HeaderValue::from_static("xxxxxxxx"));
                    tracing::info_span!("http-request", headers = ?headers)
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(AddExtensionLayer::new(adaptors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
