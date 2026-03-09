mod config;
mod error;
mod handlers;
mod models;
mod repository;
mod routes;
mod services;

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    // 初始化日志系统：cpa_card_backend=debug 确保能看到业务日志
    tracing_subscriber::fmt()
        .with_env_filter("cpa_card_backend=debug,tower_http=debug")
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    info!("✅ 数据库连接成功");

    let state = AppState { db: pool };

    let app = Router::new()
        .nest("/api/subjects", routes::subject_routes::subject_routes())
        .nest("/api/categories", routes::category_routes::category_routes())
        .nest("/api/cards", routes::card_routes::card_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http()) // 记录所有进入的 HTTP 请求日志
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 CPA Card Backend 启动于 {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}