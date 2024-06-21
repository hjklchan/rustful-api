use crate::http::{
    error::ServiceError, response::{OffsetPagination, Pagination, Response}, OhMyResult
};
use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Ticket {
    id: u64,
    title: String,
}

#[derive(Debug, Deserialize)]
pub struct Queries {
    result: u8
}

pub async fn list_handler(Query(queries): Query<Queries>) -> OhMyResult<Response<Ticket>> {
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
    
    match queries.result {
        0 => {
            Ok(Response::PaginationData(Pagination::Offset(
                OffsetPagination {
                    items: tickets,
                    page: 1,
                    size: 20,
                    total: 1,
                },
            )))
        },
        1 => {
            Err(ServiceError::SqlxError(sqlx::Error::RowNotFound))
        },
        _ => {
            Ok(Response::Ok)
        },
    }
}
