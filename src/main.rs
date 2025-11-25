mod protocol;
mod ws;

use axum::{
    Router, routing::get
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(ws::upgrade));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

