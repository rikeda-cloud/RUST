use crate::camera::camera::Camera;
use axum::extract::ws;
use futures::StreamExt;
use futures_util::SinkExt;
use opencv::prelude::VectorToVec;
use opencv::{core, imgcodecs};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Deserialize)]
struct NumberSelectEvent {
    number: i32,
}

pub async fn send_camera_frame(
    mut send_socket: futures::stream::SplitSink<ws::WebSocket, ws::Message>,
    camera: Arc<Mutex<Camera>>,
) {
    loop {
        let mut camera = camera.lock().await;
        let _ = camera.capture_frame();
        let mut buf = core::Vector::new();
        imgcodecs::imencode(".jpg", &camera.frame, &mut buf, &Default::default()).unwrap();

        // WebSocketでバイナリデータとして送信
        if send_socket
            .send(ws::Message::Binary(buf.to_vec()))
            .await
            .is_err()
        {
            break;
        }
    }
}

pub async fn recv_key_event(
    mut recv_socket: futures::stream::SplitStream<ws::WebSocket>,
    camera: Arc<Mutex<Camera>>,
) {
    while let Some(Ok(msg)) = recv_socket.next().await {
        match msg {
            ws::Message::Text(text) => {
                if let Ok(select_event) = serde_json::from_str::<NumberSelectEvent>(&text) {
                    let selected_number: i32 = select_event.number;
                    let mut camera = camera.lock().await;
                    camera.handle_key_websocket(selected_number);
                }
            }
            ws::Message::Binary(_) => {}
            ws::Message::Close(_) => break,
            _ => {}
        }
    }
}
