use crate::AppState;
use crate::handlers::card_handler;
use axum::{
    Router,
    routing::{get, patch}, // 只保留用到的
};

pub fn card_routes() -> Router<AppState> {
    Router::new()
        // 使用连写方式，这样不需要额外导入 post/put 宏
        .route("/", get(card_handler::list_cards).post(card_handler::create_card))
        .route("/search", get(card_handler::search_cards))
        .route("/by_subject", get(card_handler::list_cards_by_subject))
        .route("/:id", get(card_handler::get_card_detail).put(card_handler::update_card).delete(card_handler::delete_card))
        .route("/:id/review", patch(card_handler::review_card))
}