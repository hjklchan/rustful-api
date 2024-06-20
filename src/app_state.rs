use sqlx::{MySql, Pool};


#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<MySql>,
}