use crate::app_state::AppState;
use axum::{routing, Router};

mod create;
mod delete;
mod get;
mod list;
mod update;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/tickets", routing::post(create::create_handler))
        .route("/tickets/:ticket_id", routing::delete(delete::delete_handler))
        .route("/tickets/:ticket_id", routing::put(update::update_handler))
        .route("/tickets/:ticket_id", routing::get(get::get_handler))
        .route("/tickets", routing::get(list::list_handler))
        .with_state(app_state)
}
