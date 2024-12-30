mod create_account;
use axum::{http::StatusCode, response::IntoResponse};
pub use create_account::create_account;

// create custom error type that will return
pub struct BadRequestError(anyhow::Error);

impl IntoResponse for BadRequestError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::BAD_REQUEST.into_response()
    }
}

impl<E> From<E> for BadRequestError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
