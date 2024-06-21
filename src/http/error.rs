use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

use super::response::ErrResponse;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("couldn't find the requested resource")]
    NotFound,
    #[error("route does not exist")]
    RouteNotFound,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();

        let (status_code, code) = match self {
            Self::SqlxError(_err) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            Self::NotFound => (StatusCode::NOT_FOUND, "RESOURCE_NOT_FOUND"),
            Self::RouteNotFound => (StatusCode::NOT_FOUND, "ROUTE_NOT_FOUND"),
        };

        (status_code, ErrResponse::new(code, message)).into_response()
    }
}
