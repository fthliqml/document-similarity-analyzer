//! Application error types

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Application error types
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Too many documents: {0}, maximum allowed is 100")]
    TooManyDocuments(usize),

    #[error("Empty document at index {0}")]
    EmptyDocument(usize),

    #[error("No documents provided")]
    NoDocuments,

    #[error("Not enough documents: minimum 2 required for comparison, got {0}")]
    NotEnoughDocuments(usize),

    #[error("Document at index {0} exceeds maximum length of {1} characters")]
    DocumentTooLong(usize, usize),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

/// Error response body
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::TooManyDocuments(_) => (StatusCode::BAD_REQUEST, "TOO_MANY_DOCUMENTS"),
            AppError::EmptyDocument(_) => (StatusCode::BAD_REQUEST, "EMPTY_DOCUMENT"),
            AppError::NoDocuments => (StatusCode::BAD_REQUEST, "NO_DOCUMENTS"),
            AppError::NotEnoughDocuments(_) => (StatusCode::BAD_REQUEST, "NOT_ENOUGH_DOCUMENTS"),
            AppError::DocumentTooLong(_, _) => (StatusCode::BAD_REQUEST, "DOCUMENT_TOO_LONG"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        let body = ErrorResponse {
            error: self.to_string(),
            code: code.to_string(),
        };

        (status, Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let err = AppError::TooManyDocuments(150);
        assert!(err.to_string().contains("150"));

        let err = AppError::EmptyDocument(5);
        assert!(err.to_string().contains("5"));

        let err = AppError::NotEnoughDocuments(1);
        assert!(err.to_string().contains("1"));
    }
}
