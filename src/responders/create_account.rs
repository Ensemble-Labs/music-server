use axum::body::Bytes;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateAccount {
    name: String,
    password: String,
}

pub async fn create_account(bytes: Bytes) {
    todo!()
}
