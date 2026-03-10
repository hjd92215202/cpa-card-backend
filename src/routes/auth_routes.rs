use axum::{routing::post, Router};
use crate::handlers::auth_handler;
use crate::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
}