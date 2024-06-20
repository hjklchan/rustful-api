use std::net::SocketAddr;

use axum::Router;
use rustful_api::{app_state::AppState, db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 初始化数据库连接池
    let db_pool = db::must_connect_pool("mysql://root:@127.0.0.1:3306/test").await;
    tracing::info!("Database is connected");

    // 实例化全局状态
    let app_state = AppState {
        pool: db_pool,
    };

    // 实例化 App
    let app = Router::new().with_state(app_state);
    // 启动服务
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    let tcp_listener = TcpListener::bind(socket_addr).await.unwrap();
    tracing::info!("Listen on http://127.0.0.1:8888");
    axum::serve(tcp_listener, app).await.unwrap();
}
