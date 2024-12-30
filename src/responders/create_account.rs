use axum::body::Bytes;
use serde::Deserialize;

use crate::services::AccountService;

use super::BadRequestError;

#[derive(Deserialize)]
struct CreateAccount {
    username: String,
    password: String,
    is_admin: bool,
}

pub async fn create_account(bytes: Bytes) -> Result<(), BadRequestError> {
    let request_info: CreateAccount = bincode::deserialize(&bytes)?;
    AccountService.register(
        request_info.username,
        request_info.password,
        request_info.is_admin,
    )?;
    Ok(())
}
