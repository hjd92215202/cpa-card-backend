use crate::AppState;
use crate::handlers::subject_handler;
use axum::{
    Router,
    routing::{get, delete},
};

pub fn subject_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(subject_handler::list_subjects).post(subject_handler::create_subject),
        )
        .route("/:id", delete(subject_handler::delete_subject))
}
