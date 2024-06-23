use axum::extract::{Path, State};

use crate::{
    app_state::AppState,
    http::{error::ServiceError, response::Response, OhMyResult},
};

pub async fn delete_handler(
    State(AppState { ref pool }): State<AppState>,
    Path(article_id): Path<u64>,
) -> OhMyResult<Response<()>> {
    let sql = "UPDATE `articles` SET `deleted_at` = NOW() WHERE `id` = ?";

    sqlx::query(sql)
        .bind(article_id)
        .execute(pool)
        .await
        .map(|_| OhMyResult::Ok(Response::Ok))
        .map_err(|err| ServiceError::SqlxError(err))?
}
