//! HTTP API module

mod error;
pub mod handlers;
mod server;
mod file_upload;

pub use error::AppError;
pub use handlers::{analyze_handler, health_handler, validate_request};
pub use file_upload::analyze_files_handler;
pub use server::{create_router, run_server};
