pub mod tickets;

use axum::Router;
use crate::app_state::AppState;

pub fn routes(app_state: AppState) -> Router {
    Router::new().merge(tickets::routes(app_state))
}

pub fn router_with_state(app_state: AppState) -> Router {
    routes(app_state)
}