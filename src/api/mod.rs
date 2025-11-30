//! HTTP API module

mod error;
pub mod handlers;
mod server;

pub use error::AppError;
pub use handlers::{analyze_handler, health_handler, validate_request};
pub use server::{create_router, run_server};
