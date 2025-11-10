use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;

use super::handlers::{self, AppState};

pub async fn run_server(addr: &str) {
    let state = Arc::new(AppState { data_path: None });
    let app = create_router_with_state(state);

    let addr: SocketAddr = addr.parse().expect("Invalid address");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

pub fn create_router() -> Router {
    let state = Arc::new(AppState { data_path: None });
    create_router_with_state(state)
}

pub fn create_router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::dashboard))
        .with_state(state)
}
