use crate::camera::camera::Camera;
use crate::streaming::connections::{convert_connections_to_process_chain, Connections};
use axum::extract::ws::{Message, WebSocket};
use futures::{stream, StreamExt};
use futures_util::SinkExt;
use opencv::{core, imgcodecs, prelude::VectorToVec};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn send_camera_frame(
    mut send_socket: stream::SplitSink<WebSocket, Message>,
    camera: Arc<Mutex<Camera>>,
) {
    loop {
        let mut camera = camera.lock().await;
        let _ = camera.capture_frame();
        let mut buf = core::Vector::new();
        imgcodecs::imencode(".jpg", &camera.frame, &mut buf, &Default::default()).unwrap();

        // WebSocketでバイナリデータとして送信
        if send_socket
            .send(Message::Binary(buf.to_vec()))
            .await
            .is_err()
        {
            break;
        }
    }
}

pub async fn recv_key_event(
    mut recv_socket: stream::SplitStream<WebSocket>,
    camera: Arc<Mutex<Camera>>,
) {
    while let Some(Ok(msg)) = recv_socket.next().await {
        match msg {
            Message::Text(text) => {
                if let Ok(connections_data) = serde_json::from_str::<Connections>(&text) {
                    let camera_chain: Vec<String> =
                        convert_connections_to_process_chain(connections_data.nodes);
                    let mut camera = camera.lock().await;
                    camera.set_process_chain(camera_chain.clone());
                }
            }
            Message::Binary(_) => {}
            Message::Close(_) => break,
            _ => {}
        }
    }
}
