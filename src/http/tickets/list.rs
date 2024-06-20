use axum::response::IntoResponse;

pub async fn list_handler() -> impl IntoResponse {
    "List"
}