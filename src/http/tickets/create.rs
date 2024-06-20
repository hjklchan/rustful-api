use axum::response::IntoResponse;

pub async fn create_handler() -> impl IntoResponse {
    "Create"
}