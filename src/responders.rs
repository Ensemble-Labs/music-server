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

/// Simple macro to reduce boilerplate of trying to get a header value as a [&str].
/// Note that the calling function must return [BadRequestError] or [anyhow::Error]
/// as the macro makes two separate try calls.
///
/// Example:
/// ```rs
/// pub async fn resp(headers: HeaderMap) -> Result<(), BadRequestError> {
///     let username: &str = try_header!(headers["username"]);
///     todo!();
/// }
/// ```
#[macro_export]
macro_rules! try_header {
    ($i:ident[$h:literal]) => {
        $i.get($h).ok_or(BadRequestError::default())?.to_str()?
    };
}
