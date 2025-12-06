//! HTTP server configuration

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use super::file_upload::analyze_files_handler;
use super::handlers::health_handler;

/// Creates the Axum router with all routes configured
pub fn create_router() -> Router {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_handler))
        .route("/api/analyze", post(analyze_files_handler))
        .layer(cors)
}

/// Runs the HTTP server
pub async fn run_server(port: u16) -> anyhow::Result<()> {
    let app = create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("🚀 Server starting on http://{}", addr);
    info!("📊 POST /api/analyze - Analyze sentence-level similarity (multipart file upload)");
    info!("❤️  GET /health      - Health check");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

