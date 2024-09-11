use crate::camera::camera::Camera;
use crate::streaming::generate_response::*;
use crate::streaming::handle_websocket::*;
use axum::extract::{ws, Path};
use axum::response::IntoResponse;
use futures::StreamExt;
use phf::phf_map;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn root_handler() -> impl IntoResponse {
    static_content_handler(Path("".to_string())).await
}

pub async fn static_content_handler(Path(file): Path<String>) -> impl IntoResponse {
    static HTML_DATA_MAP: phf::Map<&'static str, &'static str> = phf_map! {
        "" => include_str!("static/index.html"),
        "index.html" => include_str!("static/index.html"),
        "websocket.js" => include_str!("static/websocket.js"),
        "streaming.css" => include_str!("static/streaming.css"),
    };
    match HTML_DATA_MAP.get(file.as_str()) {
        Some(data) => generate_text_response(data),
        None => generate_not_found_response("Error: not found html"),
    }
}

pub async fn websocket_handler(ws: ws::WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| {
        const CAMERA_NUMBER: i32 = 14;
        let camera = Arc::new(Mutex::new(Camera::new(CAMERA_NUMBER)));
        let (send_socket, recv_socket) = socket.split();

        let camera_for_recv = Arc::clone(&camera);
        tokio::spawn(async move {
            recv_key_event(recv_socket, camera_for_recv).await;
        });

        let camera_for_send = Arc::clone(&camera);
        tokio::spawn(async move {
            send_camera_frame(send_socket, camera_for_send).await;
        });
        async { () }
    })
}
