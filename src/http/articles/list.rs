use crate::{
    app_state::AppState,
    http::{
        error::ServiceError,
        response::{OffsetPagination, Pagination, Response},
        OhMyResult,
    },
    utils::pagination::{PaginationQueries, PaginationUtil},
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
    let mut pagination: PaginationUtil = queries.into();

    let sql = "SELECT COUNT(1) AS `count` FROM `articles` WHERE `deleted_at` IS NULL";
    let total_result: TotalResult = sqlx::query_as(sql)
        .fetch_one(pool)
        .await
        .map_err(|err| ServiceError::SqlxError(err))?;

    let limit_sql = pagination
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

    pagination.set_total_date_size(total_result.count as u64);
    let total_page = pagination.total_page();
    let (prev_query, next_query) = pagination.cursors();
    Ok(Response::PaginationData(Pagination::Offset(
        OffsetPagination {
            items,
            page: pagination.get_page(),
            size: pagination.get_size(),
            total: total_page,
            prev_query,
            next_query,
        },
    )))
}
