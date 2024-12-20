//! The music server backend for [insert name here].

mod auth;

use axum::{Router, routing::get};

const IP_ADDR: &str = "0.0.0.0:31078";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_responder));

    let listener = tokio::net::TcpListener::bind(IP_ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_responder() -> &'static str {
    "Hello, world!"
}
