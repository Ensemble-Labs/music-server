use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use tracing::debug;

use crate::{
    auth::manager::Token,
    services::{AccountService, SessionService},
};

use super::BadRequestError;

#[derive(Deserialize)]
struct CreateAccount {
    username: String,
    password: String,
    is_admin: bool,
}

pub async fn create_account(headers: HeaderMap, bytes: Bytes) -> Result<(), BadRequestError> {
    let request_info: CreateAccount = pot::from_slice(&bytes)?;
    let username = headers
        .get("username")
        .ok_or(BadRequestError::default())?
        .to_str()?;
    let token = headers
        .get("token")
        .ok_or(BadRequestError::default())?
        .to_str()?;

    if !SessionService.authenticate(username, &Token(uuid::Uuid::parse_str(token)?)) {
        return Err(BadRequestError(StatusCode::UNAUTHORIZED));
    }

    debug!(
        "creating account {{ username: {}, password: {} }}",
        &request_info.username, &request_info.password
    );

    AccountService.register(
        request_info.username,
        request_info.password,
        request_info.is_admin,
    )?;
    Ok(())
}
