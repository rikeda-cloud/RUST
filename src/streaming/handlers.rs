use crate::camera::camera::Camera;
use crate::streaming::generate_response::*;
use axum::extract::{ws, Path};
use axum::response::IntoResponse;
use opencv::prelude::VectorToVec;
use opencv::{core, imgcodecs};
use phf::phf_map;

pub async fn root_handler() -> impl IntoResponse {
    static_content_handler(Path("".to_string())).await
}

pub async fn static_content_handler(Path(file): Path<String>) -> impl IntoResponse {
    static HTML_DATA_MAP: phf::Map<&'static str, &'static str> = phf_map! {
        "" => include_str!("static/index.html"),
        "index.html" => include_str!("static/index.html"),
        "websocket.js" => include_str!("static/websocket.js"),
    };
    match HTML_DATA_MAP.get(file.as_str()) {
        Some(data) => generate_text_response(data),
        None => generate_not_found_response("Error: not found html"),
    }
}

pub async fn websocket_handler(ws: ws::WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        send_camera_frame(socket).await;
    })
}

async fn send_camera_frame(mut socket: ws::WebSocket) {
    const CAMERA_NUMBER: i32 = 14;
    let mut camera = Camera::new(CAMERA_NUMBER, "color");

    loop {
        let _ = camera.capture_frame();
        let mut buf = core::Vector::new();
        imgcodecs::imencode(".jpg", &camera.frame, &mut buf, &Default::default()).unwrap();

        // WebSocketでバイナリデータとして送信
        if socket
            .send(ws::Message::Binary(buf.to_vec()))
            .await
            .is_err()
        {
            break;
        }
    }
}
