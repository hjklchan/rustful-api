use axum::response::IntoResponse;

pub async fn delete_handler() -> impl IntoResponse {
    "Delete"
}