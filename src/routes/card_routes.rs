use crate::AppState;
use crate::handlers::card_handler;
use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn card_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(card_handler::list_cards))
        .route("/:id", get(card_handler::get_card_detail))
        .route("/", post(card_handler::create_card))
        .route("/search", get(card_handler::search_cards))
        .route("/:id/review", patch(card_handler::review_card))
}
