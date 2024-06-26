use axum::response::IntoResponse;

use super::{error::ServiceError, OhMyResult};

pub async fn route_not_found_handler() -> impl IntoResponse {
    OhMyResult::<()>::Err(ServiceError::RouteNotFound)
}
