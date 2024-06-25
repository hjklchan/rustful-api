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

#[derive(Debug, FromRow, Serialize)]
pub struct ListItem {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, FromRow)]
struct TotalResult {
    count: i64,
}

pub async fn list_handler(
    State(AppState { ref pool }): State<AppState>,
    Query(queries): Query<PaginationQueries>,
) -> OhMyResult<Response<ListItem>> {
    let sql = "SELECT COUNT(1) AS `count` FROM `articles` WHERE `deleted_at` IS NULL";
    let total_result: TotalResult = sqlx::query_as(sql).fetch_one(pool).await.map_err(|err| ServiceError::SqlxError(err))?;

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

    let total_page = queries.total_page(total_result.count);
    let (prev_query, next_query) = queries.page_cursors(total_page);
    Ok(Response::PaginationData(Pagination::Offset(
        OffsetPagination {
            items,
            page: queries.page(),
            size: queries.size(),
            total: total_page,
            prev_query,
            next_query,
        },
    )))
}
