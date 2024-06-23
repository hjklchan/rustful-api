use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::utils::pagination::PaginationError;

use super::response::ErrResponse;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("couldn't find the requested resource")]
    NotFound,
    #[error("route does not exist")]
    RouteNotFound,
    #[error(transparent)]
    PaginationError(#[from] PaginationError),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();

        let (status_code, code) = match self {
            Self::SqlxError(err_kind) => match err_kind {
                // TODO: Need to test
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "ROW_NOT_FOUND"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "RESOURCE_NOT_FOUND"),
            Self::RouteNotFound => (StatusCode::NOT_FOUND, "ROUTE_NOT_FOUND"),
            Self::PaginationError(_) => (StatusCode::BAD_REQUEST, "PAGINATION_ERROR"),
        };

        (status_code, ErrResponse::new(code, message)).into_response()
    }
}
