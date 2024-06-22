use crate::{
    http::{
        response::{OffsetPagination, Pagination, Response},
        OhMyResult,
    },
    utils::pagination::PaginationQueries,
};
use axum::extract::Query;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Ticket {
    id: u64,
    title: String,
}

pub async fn list_handler(
    Query(queries): Query<PaginationQueries>,
) -> OhMyResult<Response<Ticket>> {
    let _limit_sql = queries.to_sql();
    println!("{_limit_sql:?}");
    
    let tickets = vec![
        Ticket {
            id: 100,
            title: "How to make a good axum api?".to_string(),
        },
        Ticket {
            id: 104,
            title: "How to deploy axum app on ubuntu system".to_string(),
        },
    ];

    Ok(Response::PaginationData(Pagination::Offset(
        OffsetPagination {
            items: tickets,
            page: 1,
            size: 20,
            total: 1,
        },
    )))
}
