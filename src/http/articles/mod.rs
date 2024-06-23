use crate::app_state::AppState;
use axum::{routing, Router};

mod create;
mod delete;
mod get;
mod list;
mod update;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/articles", routing::post(create::create_handler))
        .route("/articles/:article_id", routing::delete(delete::delete_handler))
        .route("/articles/:article_id", routing::put(update::update_handler))
        .route("/articles/:article_id", routing::get(get::get_handler))
        .route("/articles", routing::get(list::list_handler))
        .with_state(app_state)
}
