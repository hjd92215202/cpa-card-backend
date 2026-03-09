use axum::{routing::{get, post}, Router};
use crate::handlers::subject_handler;
use crate::AppState;

pub fn subject_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(subject_handler::list_subjects))
        .route("/", post(subject_handler::create_subject))
}