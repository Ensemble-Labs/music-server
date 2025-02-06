use super::BadRequestError;
use crate::{service::auth::Token, services::SessionService, try_header, types::AuthCode};

use axum::http::{HeaderMap, StatusCode};

/// The handler function for the `/login` endpoint.
pub async fn login(headers: HeaderMap) -> Result<Token, BadRequestError> {
    let username: &str = try_header!(headers["username"]);
    let password: &str = try_header!(headers["password"]);

    match SessionService.login(username, password) {
        AuthCode::Success(session) => {
            tracing::debug!(
                "Successfully logged in {username}:{password} with token {}",
                session.token().0.to_string()
            );
            Ok(session.token())
        }
        AuthCode::InvalidPassword => Err(BadRequestError(StatusCode::UNAUTHORIZED)),
        AuthCode::AccountNotFound => Err(BadRequestError::default()),
    }
}
