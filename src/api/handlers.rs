//! API request handlers

use axum::Json;

use crate::core::analyze_documents;
use crate::models::{AnalyzeRequest, AnalyzeResponse};
use super::AppError;

/// Maximum number of documents allowed per request
const MAX_DOCUMENTS: usize = 100;

/// Minimum number of documents required for comparison
const MIN_DOCUMENTS: usize = 2;

/// Maximum length of a single document in characters
const MAX_DOCUMENT_LENGTH: usize = 50_000;

/// Handler for POST /analyze endpoint
///
/// Receives documents and returns their similarity matrix.
pub async fn analyze_handler(
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, AppError> {
    // Validate input
    validate_request(&payload)?;

    // Process documents through the pipeline
    let result = analyze_documents(&payload.documents);

    // Convert to response format
    Ok(Json(AnalyzeResponse::from(result)))
}

/// Validates the analyze request
pub fn validate_request(request: &AnalyzeRequest) -> Result<(), AppError> {
    // Check if documents array is empty
    if request.documents.is_empty() {
        return Err(AppError::NoDocuments);
    }

    // Check minimum documents
    if request.documents.len() < MIN_DOCUMENTS {
        return Err(AppError::NotEnoughDocuments(request.documents.len()));
    }

    // Check maximum documents
    if request.documents.len() > MAX_DOCUMENTS {
        return Err(AppError::TooManyDocuments(request.documents.len()));
    }

    // Check each document
    for (i, doc) in request.documents.iter().enumerate() {
        // Check for empty documents
        if doc.trim().is_empty() {
            return Err(AppError::EmptyDocument(i));
        }

        // Check document length
        if doc.len() > MAX_DOCUMENT_LENGTH {
            return Err(AppError::DocumentTooLong(i, MAX_DOCUMENT_LENGTH));
        }
    }

    Ok(())
}

/// Handler for GET /health endpoint
pub async fn health_handler() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_request() {
        let request = AnalyzeRequest::new(vec![]);
        let result = validate_request(&request);
        assert!(matches!(result, Err(AppError::NoDocuments)));
    }

    #[test]
    fn test_validate_single_document() {
        let request = AnalyzeRequest::new(vec!["hello".to_string()]);
        let result = validate_request(&request);
        assert!(matches!(result, Err(AppError::NotEnoughDocuments(1))));
    }

    #[test]
    fn test_validate_too_many_documents() {
        let docs: Vec<String> = (0..101).map(|i| format!("doc {}", i)).collect();
        let request = AnalyzeRequest::new(docs);
        let result = validate_request(&request);
        assert!(matches!(result, Err(AppError::TooManyDocuments(101))));
    }

    #[test]
    fn test_validate_empty_document() {
        let request = AnalyzeRequest::new(vec![
            "hello".to_string(),
            "   ".to_string(), // Empty after trim
            "world".to_string(),
        ]);
        let result = validate_request(&request);
        assert!(matches!(result, Err(AppError::EmptyDocument(1))));
    }

    #[test]
    fn test_validate_valid_request() {
        let request = AnalyzeRequest::new(vec![
            "hello world".to_string(),
            "foo bar".to_string(),
        ]);
        let result = validate_request(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_max_documents() {
        let docs: Vec<String> = (0..100).map(|i| format!("document {}", i)).collect();
        let request = AnalyzeRequest::new(docs);
        let result = validate_request(&request);
        assert!(result.is_ok());
    }
}
