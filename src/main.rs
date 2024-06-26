use std::{net::SocketAddr, str::FromStr};

use axum::http::Method;
use rustful_api::{app_state::AppState, db, http, settings};
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};

#[tokio::main]
async fn main() {
    // 初始化配置
    let settings = settings::Settings::new().unwrap();

    // 初始化日志
    tracing_subscriber::fmt::init();

    // 初始化数据库连接池
    let db_pool = db::must_connect_pool(settings.database.url).await;
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
    let socket_addr = SocketAddr::from_str(settings.server.addr.as_str()).unwrap();
    let tcp_listener = TcpListener::bind(socket_addr).await.unwrap();
    tracing::info!("Listen on http://{}", socket_addr);
    print_ascii_logo();
    axum::serve(tcp_listener, app).await.unwrap();
}

fn print_ascii_logo() -> () {
    static LOGO: &str = r#"
    /$$$$$$$ /$$   /$$ /$$$$$$ /$$$$$$$$/$$$$$$$$/$$   /$$/$$      
    | $$__  $| $$  | $$/$$__  $|__  $$__| $$_____| $$  | $| $$      
    | $$  \ $| $$  | $| $$  \__/  | $$  | $$     | $$  | $| $$      
    | $$$$$$$| $$  | $|  $$$$$$   | $$  | $$$$$  | $$  | $| $$      
    | $$__  $| $$  | $$\____  $$  | $$  | $$__/  | $$  | $| $$      
    | $$  \ $| $$  | $$/$$  \ $$  | $$  | $$     | $$$$| $| $$$$$$$$
    |__/  |__/\______/ \______/   |__/  |__/      \______/|________/

    > Github: https://github.com/hjklchan/rustful-api
    "#;

    println!("{LOGO}");
}
