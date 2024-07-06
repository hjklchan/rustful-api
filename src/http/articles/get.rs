use axum::{extract::{Path, State}};
use chrono::Local;
use serde::Serialize;

use crate::{
    app_state::AppState,
    http::{error::ServiceError, response::Response, OhMyResult},
    model::ticket::Ticket,
};

#[derive(Serialize)]
pub struct GetArticleReply {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
    pub created_at: chrono::DateTime<Local>,
    pub updated_at: chrono::DateTime<Local>,
}

impl From<Ticket> for GetArticleReply {
    fn from(
        Ticket {
            id,
            title,
            description,
            body,
            created_at,
            updated_at,
            ..
        }: Ticket,
    ) -> Self {
        Self {
            id,
            title,
            description,
            body,
            created_at,
            updated_at,
        }
    }
}

pub async fn get_handler(
    State(AppState { ref pool }): State<AppState>,
    Path(ticket_id): Path<u64>,
) -> OhMyResult<Response<GetArticleReply>> {
    let maybe_ticket: Option<Ticket> = sqlx::query_as("SELECT `id`, `title`, `description`, `body`, `created_at`, `updated_at`, `deleted_at` FROM `tickets` WHERE `id` = ?").bind(ticket_id).fetch_optional(pool).await.map_err(|err| ServiceError::SqlxError(err))?;

    if let Some(value) = maybe_ticket {
        return OhMyResult::Ok(Response::Data(GetArticleReply::from(value)));
    }

    return OhMyResult::Err(ServiceError::NotFound);
}
