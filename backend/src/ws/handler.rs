use crate::protocol::{Command, DashboardCommand, DashboardTelemetry, Telemetry};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc::unbounded_channel;

pub async fn esp_upgrade(
    ws: WebSocketUpgrade,
    state: axum::extract::State<crate::AppState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| async move {
        handle_socket(socket, state).await;
    })
}
pub async fn dashboard_upgrade(
    ws: WebSocketUpgrade,
    state: axum::extract::State<crate::AppState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_dashboard_socket(socket, state))
}
async fn handle_socket(socket: WebSocket, state: axum::extract::State<crate::AppState>) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (tx, mut rx) = unbounded_channel::<Message>();
    *state.esp_tx.write().unwrap() = Some(tx);

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
            Message::Binary(b) => match Telemetry::from_bytes(&b) {
                Ok(t) => {
                    let dashboards = state.dashboards.read().unwrap();

                    for tx in dashboards.iter() {
                        let dt = DashboardTelemetry::from(&t);
                        match serde_json::to_string(&dt) {
                            Ok(dt) => {
                                let _ = tx.send(Message::Text(dt.into()));
                            }
                            Err(err) => {
                                eprintln!("serialization error: {}", err);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("parse error: {}", e),
            },
            Message::Text(text) => println!("Received text: {}", text),
            Message::Close(_) => {
                println!("client disconnected");
                return;
            }
            _ => {}
        }
    }
    *state.esp_tx.write().unwrap() = None;
}

async fn handle_dashboard_socket(socket: WebSocket, state: axum::extract::State<crate::AppState>) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (tx, mut rx) = unbounded_channel::<Message>();
    {
        let mut list = state.dashboards.write().unwrap();
        list.push(tx);
    }

    let state_for_drop = state.clone();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                println!("dashboard websocket closed (writer)");
                break;
            }
        }

        let mut list = state_for_drop.dashboards.write().unwrap();
        list.retain(|_sender| false);
    });

    while let Some(Ok(msg)) = ws_rx.next().await {
        match msg {
            Message::Text(text) => match serde_json::from_str::<DashboardCommand>(&text) {
                Ok(dash_cmd) => {
                    println!(
                        "Received command: left {}, right {}",
                        dash_cmd.left_pwm, dash_cmd.right_pwm
                    );
                    let cmd = Command::from(&dash_cmd);
                    let cmd_bytes = cmd.to_bytes();
                    let guard = state.esp_tx.read().unwrap();
                    if let Some(tx) = guard.as_ref() {
                        if tx.send(Message::Binary(cmd_bytes.into())).is_ok() {
                            println!(
                                "Sent command: L={} R={}",
                                dash_cmd.left_pwm, dash_cmd.right_pwm
                            );
                        }
                    }
                }
                Err(e) => eprintln!("failed to parse json {}", e),
            },
            Message::Close(_) => {
                println!("dashboard disconnected");
                break;
            }
            _ => {}
        }
    }
}
