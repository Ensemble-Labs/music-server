use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use tracing::debug;

use crate::{
    auth::manager::Token,
    services::{AccountService, SessionService},
    try_header,
};

use super::BadRequestError;

/// A data struct to represent the account information sent by the binary file.
/// Any information received by this endpoint is expected to be encoded using
/// the `pot` library in this specific struct format.
#[derive(Deserialize)]
struct CreateAccount {
    username: String,
    password: String,
    is_admin: bool,
}

/// The handler function for the `/create-account` endpoint.
pub async fn create_account(headers: HeaderMap, bytes: Bytes) -> Result<(), BadRequestError> {
    let request_info: CreateAccount = pot::from_slice(&bytes)?;
    let username: &str = try_header!(headers["username"]);
    let token: &str = try_header!(headers["auth-token"]);

    if !SessionService.authenticate(username, &Token::try_from(token)?) {
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
