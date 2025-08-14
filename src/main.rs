use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Msg {
    message: String,
}

async fn health() -> &'static str {
    "ok"
}
async fn hello() -> Json<Msg> {
    Json(Msg {
        message: "Hello, MiniPC".into(),
    })
}
async fn echo(Json(payload): Json<Msg>) -> Json<Msg> {
    let message = format!("{}, host pc", payload.message);
    Json(Msg{
        message
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello))
        .route("/echo", post(echo));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
