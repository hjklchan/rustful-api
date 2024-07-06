use sqlx::{prelude::FromRow, types::chrono::{self, Local}};

#[derive(Debug, FromRow)]
#[allow(unused)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
    pub created_at: chrono::DateTime<Local>,
    pub updated_at: chrono::DateTime<Local>,
    pub deleted_at: chrono::DateTime<Local>,
}