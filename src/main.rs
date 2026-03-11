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
use dotenvy::dotenv;

/// 全局共享状态
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt_secret: String, // 用于 JWT 的签发和验证
}

#[tokio::main]
async fn main() {
    // 1. 加载环境变量
    dotenv().ok();

    // 2. 初始化日志系统
    // 默认展示项目和网络框架的 debug 信息，方便调试请求耗时
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    // 3. 读取配置
    let db_url = std::env::var("DATABASE_URL")
        .expect("必须在 .env 中设置 DATABASE_URL");
    
    // 核心安全：读取 JWT 密钥
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("必须在 .env 中设置 JWT_SECRET (建议使用随机长字符串)");

    // 4. 初始化数据库连接池
    let pool = PgPoolOptions::new()
        .max_connections(5) // CPA 学习系统并发量不大，5个连接绰绰有余
        .connect(&db_url)
        .await
        .expect("无法连接到 PostgreSQL 数据库");

    info!("✅ 数据库连接成功");

    // 5. 构建全局状态
    let state = AppState { 
        db: pool,
        jwt_secret,
    };

    // 6. 聚合路由与中间件
    let app = Router::new()
        // 用户认证模块 (登录/注册)
        .nest("/api/auth", routes::auth_routes::auth_routes())
        // 科目管理模块
        .nest("/api/subjects", routes::subject_routes::subject_routes())
        // 章节管理模块
        .nest("/api/categories", routes::category_routes::category_routes())
        // 卡片管理核心模块
        .nest("/api/cards", routes::card_routes::card_routes())
        // 中间件：跨域处理
        .layer(CorsLayer::permissive())
        // 中间件：请求追踪（日志中会显示所有接口的请求耗时）
        .layer(TraceLayer::new_for_http())
        // 注入状态
        .with_state(state);

    // 7. 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("🚀 CPA Card Backend 已就绪");
    info!("📍 监听地址: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}