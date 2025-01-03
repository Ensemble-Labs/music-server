//! the music server backend for the Orpheus project.
//! the main.rs file contains the binary part of the application, i.e.
//! the code for the main function and any relevant details.

use std::time::Duration;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, info, Level};
// import exports defined in `src/lib.rs`:
use orpheus::{
    responders,
    services::{AccountService, Config},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt() // set up global tracing
        // note that tracing doesn't even get compiled in release mode so no need
        // to optimize this setting
        .with_max_level(Level::TRACE) // set maximum log level to be outputted
        .with_level(true) // show log level in output
        .without_time() // remove timestamp from log messages
        .init();

    AccountService.verify(); // Load account service ahead of time
    let lock = Config.try_read().unwrap(); // gain a read lock over config temporarily
    let port: &str = lock.server().bind_address(); // obtain port to bind to from Config service

    let app = Router::new()
        .layer(TraceLayer::new_for_http()) // makes debugging in async frameworks tear-free!
        .layer(CorsLayer::permissive()) // idk what cors even does but it ruins my life
        .route("/", get(root_responder)) // mostly to test logging and firewalls
        .route(
            "/create-account",
            get(async || StatusCode::METHOD_NOT_ALLOWED), // explicitly disallow get requests as we need binary data
        )
        .route("/create-account", post(responders::create_account));

    std::thread::spawn(|| loop {
        // spawn a separate thread to infinitely loop and save registry if necessary
        let account_service = AccountService.as_ref();
        if account_service.is_dirty() {
            // if account registry changed since last write
            debug!("accounts service is marked dirty, autosaving...");
            account_service.save();
        }
        std::thread::sleep(Duration::from_secs(1));
    });

    let listener = tokio::net::TcpListener::bind(port)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {port}!"));
    info!("Listening on {}...", port);
    drop(lock);
    axum::serve(listener, app).await.unwrap();
    info!("Exiting gracefully...");
}

async fn root_responder() -> Result<(), StatusCode> {
    tracing::debug!("root response");
    Ok(())
}
