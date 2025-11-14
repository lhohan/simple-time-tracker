use axum::{routing::get, Router};
use std::sync::Arc;

use super::handlers::{self, AppState};

pub fn create_router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::dashboard))
        .route("/flag-statistics", get(handlers::flag_statistics))
        .route("/outcomes", get(handlers::outcomes_page))
        .route("/health", get(handlers::health_check))
        .route("/api/dashboard", get(handlers::dashboard_partial))
        .route("/api/dashboard/summary", get(handlers::dashboard_summary))
        .route("/api/flag-statistics", get(handlers::flag_statistics_partial))
        .route("/api/outcomes", get(handlers::outcomes_partial))
        .route("/api/outcomes/summary", get(handlers::outcomes_summary))
        .route("/api/tag/:tag_name", get(handlers::tag_detail))
        .route("/api/chart/projects-pie", get(handlers::chart_projects_pie))
        .route("/api/chart/outcomes-pie", get(handlers::chart_outcomes_pie))
        .with_state(state)
}
