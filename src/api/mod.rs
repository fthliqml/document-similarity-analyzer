//! HTTP API module

mod error;
mod server;
mod file_upload;

pub use error::AppError;
pub use file_upload::{analyze_files_handler, health_handler};
pub use server::{create_router, run_server};
