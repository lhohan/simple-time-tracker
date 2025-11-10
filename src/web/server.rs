use axum::{routing::get, Router};
use std::sync::Arc;

use super::handlers::{self, AppState};

pub fn create_router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::dashboard))
        .route("/health", get(handlers::health_check))
        .route("/api/dashboard", get(handlers::dashboard_partial))
        .route("/api/tag/:tag_name", get(handlers::tag_detail))
        .route("/api/chart/projects-bar", get(handlers::chart_projects_bar))
        .route("/api/chart/projects-pie", get(handlers::chart_projects_pie))
        .with_state(state)
}
