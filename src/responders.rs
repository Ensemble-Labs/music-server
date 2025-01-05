mod create_account;
use axum::{http::StatusCode, response::IntoResponse};
pub use create_account::create_account;

// A custom error type that will return bad request when returned.
pub struct BadRequestError(StatusCode);

impl Default for BadRequestError {
    fn default() -> Self {
        Self(StatusCode::BAD_REQUEST)
    }
}

impl IntoResponse for BadRequestError {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}

impl<E> From<E> for BadRequestError
where
    E: Into<anyhow::Error>,
{
    fn from(_: E) -> Self {
        Self(StatusCode::BAD_REQUEST)
    }
}
