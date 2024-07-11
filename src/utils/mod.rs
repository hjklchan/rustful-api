use axum::response::IntoResponse;
use reqwest::StatusCode;

pub mod jwt;
pub mod pagination;

pub struct Codes(StatusCode, &'static str);

impl From<(StatusCode, &'static str)> for Codes {
    fn from((status_code, code): (StatusCode, &'static str)) -> Self {
        Self(status_code, code)
    }
}

impl IntoResponse for Codes {
    fn into_response(self) -> axum::response::Response {
        // todo: transform to (status_code, ErrResponse::new(code, message)).into_response()
        unimplemented!()
    }
}
