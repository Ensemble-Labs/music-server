//! The music server backend for [insert name here].

#![feature(try_blocks)]

mod auth;
mod db;

use axum::{Router, routing::get};

const IP_ADDR: &str = "0.0.0.0:31078";

#[tokio::main]
async fn main() {
    let app = Router::new()
        // more routes later
        .route("/", get(root_responder));

    let listener = tokio::net::TcpListener::bind(IP_ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_responder() -> &'static str {
    "Hello, world!"
}
