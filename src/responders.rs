mod create_account;
use axum::{http::StatusCode, response::IntoResponse};
pub use create_account::create_account;

// A custom error type that will return bad request when returned.
pub struct BadRequestError;

impl IntoResponse for BadRequestError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::BAD_REQUEST.into_response()
    }
}

impl<E> From<E> for BadRequestError
where
    E: Into<anyhow::Error>,
{
    fn from(_: E) -> Self {
        Self
    }
}
