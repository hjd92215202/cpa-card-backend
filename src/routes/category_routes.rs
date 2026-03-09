use axum::{routing::{get, post}, Router};
use crate::handlers::category_handler;
use crate::AppState;

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(category_handler::list_by_subject))
        .route("/", post(category_handler::create_category))
}