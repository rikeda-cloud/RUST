mod camera;
mod streaming;
use axum::{routing::get, Router};
use streaming::handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::root_handler))
        .route("/ws", get(handlers::websocket_handler))
        .route("/:file", get(handlers::static_content_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
