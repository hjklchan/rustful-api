pub mod claims;
pub mod extract;

use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("missing token")]
    MissingToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        unimplemented!()
    }
}
