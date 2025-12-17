use crate::state::AppState;
use axum::{
    self, Router,
    routing::{get, post},
};

use tower_http::cors::CorsLayer;

use crate::handlers::{health, publish_message};

pub async fn create_router(state: AppState) -> Router {
    let router = axum::Router::new()
        .route("/send_message", post(publish_message))
        .route("/health", get(health))
        .layer(CorsLayer::permissive())
        .with_state(state);

    router
}
