use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    http::{error::ServiceError, response::Response, OhMyResult},
};

#[derive(Debug, Deserialize)]
pub struct CreatePayload {
    title: String,
    description: Option<String>,
    body: Option<String>,
}

pub async fn create_handler(
    State(AppState { ref pool }): State<AppState>,
    Json(req): Json<CreatePayload>,
) -> OhMyResult<Response> {
    let sql = r#"
        INSERT INTO `articles` ( `title`, `description`, `body`, `created_at`, `updated_at` )
        VALUES ( ?, ?, ?, NOW(), NOW() )
    "#;

    sqlx::query(sql)
        .bind(req.title)
        .bind(req.description)
        .bind(req.body)
        .execute(pool)
        .await
        .map_err(|err| ServiceError::SqlxError(err))
        .map(|_| {})?;

    Ok(Response::Created)
}
