use std::net::SocketAddr;

use axum::http::Method;
use rustful_api::{app_state::AppState, db, http};
use tokio::net::TcpListener;
use tower_http::cors::{self, Cors, CorsLayer};

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 初始化数据库连接池
    let db_pool = db::must_connect_pool("mysql://root:@127.0.0.1:3306/test").await;
    tracing::info!("Database is connected");

    // 实例化全局状态
    let app_state = AppState { pool: db_pool };

    // 实例化 App
    let app = http::router_with_state(app_state).layer(
        // 跨域中间件
        CorsLayer::new().allow_origin(cors::Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]),
    );
    // 启动服务
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    let tcp_listener = TcpListener::bind(socket_addr).await.unwrap();
    tracing::info!("Listen on http://127.0.0.1:8888");
    axum::serve(tcp_listener, app).await.unwrap();
}
