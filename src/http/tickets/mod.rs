use crate::app_state::AppState;
use axum::{routing, Router};

mod create;
mod delete;
mod get;
mod list;
mod update;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", routing::post(create::create_handler))
        .route("/:ticket_id", routing::delete(delete::delete_handler))
        .route("/:ticket_id", routing::put(update::update_handler))
        .route("/:ticket_id", routing::get(get::get_handler))
        .route("/", routing::get(list::list_handler))
        .with_state(app_state)
}
