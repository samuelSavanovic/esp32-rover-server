use dioxus::{
    hooks::{use_resource, use_signal},
    signals::{ReadSignal, WritableExt},
};
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use tracing::info;

pub struct CarConnection {
    pub distance: ReadSignal<Option<f32>>,
    pub send_command: Box<dyn Fn(String)>,
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
                        if let Ok(v) = text.parse::<f32>() {
                            distance.set(Some(v / 10.0));
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
        move |cmd: String| {
            if let Some(tx) = tx_signal() {
                // best-effort; use send_blocking if you want to wait
                let _ = tx.try_send(cmd.to_string());
            }
        }
    };

    CarConnection {
        distance: distance.into(),
        send_command: Box::new(send_command),
    }
}
