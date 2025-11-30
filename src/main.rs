//! Document Similarity Analyzer - Main Entry Point
//!
//! A backend service for analyzing document similarity using TF-IDF
//! and Cosine Similarity with parallel processing.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use document_similarity_analyzer::api::run_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "document_similarity_analyzer=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get port from environment or use default
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Run the server
    run_server(port).await
}
