mod protocol;
mod ws;

use std::{collections::HashMap, sync::{Arc, RwLock}};

use axum::{
    Router, extract::{Query, State, ws::Message}, routing::get
};
use tokio::sync::mpsc::UnboundedSender;

use crate::protocol::Command;
#[derive(Clone)]
struct AppState{
    tx: Arc<RwLock<Option<UnboundedSender<Message>>>>
}

#[tokio::main]
async fn main() {
    let state = AppState {  tx: Arc::new(RwLock::new(None)) };
    let app = Router::new().route("/ws", get(ws::upgrade)).route("/cmd", get(send_cmd)).with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn send_cmd(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> String {
    let l: i16 = params.get("l").and_then(|v| v.parse().ok()).unwrap_or(0);
    let r: i16 = params.get("r").and_then(|v| v.parse().ok()).unwrap_or(0);

    let cmd_bytes = Command::new(l, r).to_bytes();
    let msg = Message::Binary(cmd_bytes.into());

    let guard = state.tx.read().unwrap();
    if let Some(tx) = guard.as_ref() {
        if tx.send(msg).is_ok() {
            return format!("Sent command: L={} R={}", l, r);
        }
    }

    "No ESP32 connected".into()
}
