use dioxus::{
    hooks::{use_resource, use_signal},
    signals::{ReadSignal, WritableExt},
};
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize)]
pub struct DashboardTelemetry {
    #[allow(dead_code)]
    pub kind: u8,
    pub distance_mm: u32,
}
pub struct CarConnection {
    pub distance: ReadSignal<Option<f32>>,
    pub send_command: Box<dyn Fn(DashboardCommand)>,
}

#[derive(Serialize)]
pub struct DashboardCommand {
    pub kind: u8,
    pub left_pwm: i16,
    pub right_pwm: i16,
}

impl DashboardCommand {
    pub fn new(left_pwm: i16, right_pwm: i16) -> Self {
        Self {
            kind: 0x02,
            left_pwm,
            right_pwm,
        }
    }
}

pub fn use_car_ws(url: &'static str) -> CarConnection {
    let mut distance = use_signal(|| None::<f32>);
    let mut tx_signal = use_signal(|| None::<async_channel::Sender<String>>);

    let _ws_resource = use_resource(move || async move {
        info!("Connecting WebSocket to {url}");
        let ws = match WebSocket::open(url) {
            Ok(ws) => ws,
            Err(err) => {
                info!("WS connect error: {err:?}");
                return;
            }
        };

        let (mut write, mut read) = ws.split();

        let (tx, rx) = async_channel::unbounded::<String>();
        tx_signal.set(Some(tx));

        let read_task = async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<DashboardTelemetry>(&text) {
                            Ok(dt) => {
                                distance.set(Some(dt.distance_mm as f32 / 10.0));
                            }
                            Err(e) => info!("Deserialize error {:?}", e),
                        }
                    }
                    Ok(Message::Bytes(_)) => {
                        info!("Received bytes")
                    }
                    Err(e) => {
                        info!("WS read error: {e:?}");
                        break;
                    }
                }
            }
        };

        let write_task = async move {
            while let Ok(cmd) = rx.recv().await {
                info!("WS -> {cmd}");
                if let Err(e) = write.send(Message::Text(cmd)).await {
                    info!("WS send error: {e:?}");
                    break;
                }
            }
        };

        futures_util::future::join(read_task, write_task).await;
        info!("WebSocket closed");
    });

    let send_command = {
        move |cmd: DashboardCommand| {
            if let Some(tx) = tx_signal() {
                let _ = tx.try_send(serde_json::to_string(&cmd).unwrap());
            }
        }
    };

    CarConnection {
        distance: distance.into(),
        send_command: Box::new(send_command),
    }
}
