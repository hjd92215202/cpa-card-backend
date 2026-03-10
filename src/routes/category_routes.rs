use crate::AppState;
use crate::handlers::category_handler;
use axum::{
    Router,
    routing::{get, delete},
};

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(category_handler::list_by_subject).post(category_handler::create_category),
        )
        .route("/:id", delete(category_handler::delete_category)) // 新增删除路由
}
