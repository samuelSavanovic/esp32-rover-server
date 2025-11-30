use axum::extract::ws::{WebSocketUpgrade, WebSocket, Message};
use tokio::sync::mpsc::unbounded_channel;
use crate::protocol::Telemetry;
use futures_util::{sink::SinkExt, stream::StreamExt};

pub async fn upgrade(ws: WebSocketUpgrade,
    state: axum::extract::State<crate::AppState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| async move {
        handle_socket(socket, state).await;
    })
}
async fn handle_socket(socket: WebSocket,  state: axum::extract::State<crate::AppState>) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (tx, mut rx) = unbounded_channel::<Message>();
    *state.tx.write().unwrap() = Some(tx);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                println!("ESP32 websocket closed");
                break;
            }
        }
    });

    while let Some(Ok(msg)) = ws_rx.next().await {
        match msg {
            Message::Binary(b) => {
                match Telemetry::from_bytes(&b) {
                    Ok(t) => {
                        let dist:u32=t.distance_mm;
                        println!("Received telemetry distance_mm: {}", dist);
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
    *state.tx.write().unwrap() = None;
}
