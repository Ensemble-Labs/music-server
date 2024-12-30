//! the music server backend for the Orpheus project.
//! the main.rs file contains the binary part of the application, i.e.
//! the code for the main function and any relevant details.

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
// import exports defined in `src/lib.rs`:
use orpheus::responders;

// we want port 31078 over all interfaces (0.0.0.0)
const IP_ADDR: &str = "0.0.0.0:31078";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .layer(TraceLayer::new_for_http()) // makes debugging in async frameworks tear-free!
        .layer(CorsLayer::permissive()) // idk what cors even does but it ruins my life
        .route("/", get(root_responder))
        .route("/create-account", get(async || StatusCode::BAD_REQUEST)) // explicitly disallow get requests as we need binary data
        .route("/create-account", post(responders::create_account));

    let listener = tokio::net::TcpListener::bind(IP_ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_responder() -> &'static str {
    "Hello, world!"
}
