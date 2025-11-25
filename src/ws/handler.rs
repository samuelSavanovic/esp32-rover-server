use axum::extract::ws::{WebSocketUpgrade, WebSocket, Message};
use crate::protocol::Telemetry;

pub async fn upgrade(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Binary(b) => {
                match Telemetry::from_bytes(&b) {
                    Ok(t) => {
                        let dist:u32=t.distance_mm;
                        println!("Received telemetry distance_mm: {}", dist)
                    }
                    Err(e) => println!("parse error: {}", e),
                }
            }
            Message::Text(text) => println!("Received text: {}", text),
            Message::Close(_) => {
                println!("client disconnected");
                return;
            }
            _ => {}
        }
    }
}
