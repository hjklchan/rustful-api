use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 实例化 App
    let app = Router::new();
    // 启动服务
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    let tcp_listener = TcpListener::bind(socket_addr).await.unwrap();
    tracing::info!("Listen on http://127.0.0.1:8888");
    axum::serve(tcp_listener, app).await.unwrap();
}
