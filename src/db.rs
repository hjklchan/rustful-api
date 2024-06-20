use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

/// ### 数据库连接
/// 
/// 连接失败直接 Panic
pub async fn must_connect_pool(database_url: impl AsRef<str>) -> Pool<MySql> {
    connect_pool(database_url).await.unwrap()
}

/// ### 数据库连接
pub async fn connect_pool(database_url: impl AsRef<str>) -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new().connect(database_url.as_ref()).await
}
