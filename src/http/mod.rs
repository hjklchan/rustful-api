pub mod error;
pub mod response;
pub mod tickets;

use crate::app_state::AppState;
use axum::Router;
use error::ServiceError;

pub type OhMyResult<T> = Result<T, ServiceError>;

pub fn routes(app_state: AppState) -> Router {
    Router::new().merge(tickets::routes(app_state))
}

pub fn router_with_state(app_state: AppState) -> Router {
    routes(app_state)
}