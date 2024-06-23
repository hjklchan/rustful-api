use crate::{
    app_state::AppState,
    http::{
        error::ServiceError,
        response::{OffsetPagination, Pagination, Response},
        OhMyResult,
    },
    utils::pagination::PaginationQueries,
};
use axum::extract::{Query, State};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize)]
pub struct Ticket {
    id: u64,
    title: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct ListItem {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub async fn list_handler(
    State(AppState { ref pool }): State<AppState>,
    Query(queries): Query<PaginationQueries>,
) -> OhMyResult<Response<ListItem>> {
    let limit_sql = queries
        .to_sql()
        .map_err(|err| ServiceError::PaginationError(err))?;

    let sql = format!(
        "SELECT `id`, `title`, `description` FROM `articles` WHERE `deleted_at` IS NULL {}",
        limit_sql
    );

    let items: Vec<ListItem> =
        sqlx::query_as(&sql)
            .fetch_all(pool)
            .await
            .map_err(|err| match err {
                err => ServiceError::SqlxError(err),
            })?;

    Ok(Response::PaginationData(Pagination::Offset(
        OffsetPagination {
            items,
            page: 1,
            size: 10,
            total: 1,
        },
    )))
}
