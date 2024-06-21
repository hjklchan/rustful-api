use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::Serialize;

pub enum Response<T> {
    Ok,
    Created,
    Data(T),
    PaginationData(Pagination<T>),
}

impl<T> IntoResponse for Response<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            // 普通数据
            Self::Data(data) => (StatusCode::OK, Json(data)).into_response(),
            // 分页数据
            Self::PaginationData(pagination) => match pagination {
                Pagination::Cursor(data) => (StatusCode::OK, Json(data)).into_response(),
                Pagination::Offset(data) => (StatusCode::OK, Json(data)).into_response(),
            },
        }
    }
}

pub enum Pagination<T> {
    Cursor(CursorPagination<T>),
    Offset(OffsetPagination<T>),
}

#[derive(Debug, Serialize)]
pub struct CursorPagination<T> {
    items: Vec<T>,
    // other fields...
}

#[derive(Debug, Serialize)]
pub struct OffsetPagination<T> {
    pub items: Vec<T>,
    pub page: u64,
    pub size: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct ErrResponse {
    pub code: &'static str,
    pub message: String,
}

impl ErrResponse {
    pub fn new(code: &'static str, message: String) -> Self {
        Self { code, message }
    }
}

impl IntoResponse for ErrResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
