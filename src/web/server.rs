use axum::{routing::get, Router};
use std::sync::Arc;

use super::handlers::{self, AppState};

pub fn create_router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::dashboard))
        .route("/api/dashboard", get(handlers::dashboard_partial))
        .route("/api/tag/:tag_name", get(handlers::tag_detail))
        .with_state(state)
}
