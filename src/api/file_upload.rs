//! File upload handler for sentence-level analysis

use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::time::Instant;

use crate::extraction::{extract_text, FileType};
use crate::sentence::split_sentences;
use crate::core::{analyze_sentence_similarity, SentenceDocument};
use crate::models::{SentenceAnalysisResponse, AnalysisMetadata};

/// Constants for file upload limits
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MB
const MAX_TOTAL_SIZE: usize = 50 * 1024 * 1024; // 50 MB
const MAX_FILES: usize = 5;
const MIN_FILES: usize = 2;
const DEFAULT_THRESHOLD: f32 = 0.70;

/// Handler for POST /api/analyze with multipart file upload
///
/// Accepts up to 5 files (PDF/DOCX/TXT) and returns sentence-level similarity analysis.
pub async fn analyze_files_handler(
    mut multipart: Multipart,
) -> Result<Json<SentenceAnalysisResponse>, FileUploadError> {
    let start_time = Instant::now();
    
    // Collect files and threshold from multipart form
    let (files, threshold) = extract_files_and_threshold(&mut multipart).await?;

    // Validate minimum files
    if files.len() < MIN_FILES {
        return Err(FileUploadError::NotEnoughFiles(MIN_FILES));
    }

    // Extract text from files
    let documents: Result<Vec<SentenceDocument>, FileUploadError> = files
        .into_iter()
        .map(|(filename, data)| {
            // Detect file type
            let file_type = FileType::from_filename(&filename)
                .ok_or_else(|| FileUploadError::UnsupportedFileType(filename.clone()))?;

            // Extract text
            let text = extract_text(&data, file_type)
                .map_err(|e| FileUploadError::ExtractionError(filename.clone(), e))?;

            // Split into sentences
            let sentences = split_sentences(&text);

            if sentences.is_empty() {
                return Err(FileUploadError::EmptyDocument(filename));
            }

            Ok(SentenceDocument::new(filename, sentences))
        })
        .collect();

    let documents = documents?;

    // Count total sentences
    let total_sentences: usize = documents.iter().map(|d| d.sentences.len()).sum();

    // Analyze similarity
    let (matches, global_similarity) = analyze_sentence_similarity(&documents, threshold);

    // Compute processing time
    let processing_time_ms = start_time.elapsed().as_millis() as u64;

    // Build metadata
    let metadata = AnalysisMetadata::new(
        documents.len(),
        total_sentences,
        processing_time_ms,
        threshold,
    );

    // Build response
    let response = SentenceAnalysisResponse::new(metadata, matches, global_similarity);

    Ok(Json(response))
}

/// Errors that can occur during file upload and processing
#[derive(Debug)]
pub enum FileUploadError {
    InvalidMultipart(String),
    MissingFilename,
    ReadError(String),
    FileTooLarge(String, usize),
    TotalSizeTooLarge(usize),
    TooManyFiles(usize),
    NotEnoughFiles(usize),
    UnsupportedFileType(String),
    ExtractionError(String, String),
    EmptyDocument(String),
    InvalidThreshold(String),
    InvalidThresholdRange(f32),
}

impl IntoResponse for FileUploadError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            FileUploadError::InvalidMultipart(e) => {
                (StatusCode::BAD_REQUEST, format!("Invalid multipart data: {}", e))
            }
            FileUploadError::MissingFilename => {
                (StatusCode::BAD_REQUEST, "File is missing filename".to_string())
            }
            FileUploadError::ReadError(e) => {
                (StatusCode::BAD_REQUEST, format!("Error reading file: {}", e))
            }
            FileUploadError::FileTooLarge(filename, max) => {
                (
                    StatusCode::PAYLOAD_TOO_LARGE,
                    format!("File '{}' exceeds maximum size of {} bytes", filename, max),
                )
            }
            FileUploadError::TotalSizeTooLarge(max) => {
                (
                    StatusCode::PAYLOAD_TOO_LARGE,
                    format!("Total upload size exceeds maximum of {} bytes", max),
                )
            }
            FileUploadError::TooManyFiles(max) => {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Too many files. Maximum allowed: {}", max),
                )
            }
            FileUploadError::NotEnoughFiles(min) => {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Not enough files. Minimum required: {}", min),
                )
            }
            FileUploadError::UnsupportedFileType(filename) => {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Unsupported file type: {}. Allowed: PDF, DOCX, TXT", filename),
                )
            }
            FileUploadError::ExtractionError(filename, error) => {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!("Failed to extract text from '{}': {}", filename, error),
                )
            }
            FileUploadError::EmptyDocument(filename) => {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Document '{}' contains no text or sentences", filename),
                )
            }
            FileUploadError::InvalidThreshold(value) => {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid threshold value: '{}'. Must be a number between 0.0 and 1.0", value),
                )
            }
            FileUploadError::InvalidThresholdRange(value) => {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Threshold {} out of range. Must be between 0.0 and 1.0", value),
                )
            }
        };

        (status, message).into_response()
    }
}

/// Extract files and threshold from multipart form data
async fn extract_files_and_threshold(
    multipart: &mut Multipart,
) -> Result<(Vec<(String, Vec<u8>)>, f32), FileUploadError> {
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    let mut threshold_value: Option<f32> = None;
    let mut total_size = 0usize;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| FileUploadError::InvalidMultipart(e.to_string()))? {
        
        let field_name = field.name().unwrap_or("").to_string();
        
        // Check if this is the threshold field
        if field_name == "threshold" {
            let threshold_str = field.text().await
                .map_err(|e| FileUploadError::ReadError(e.to_string()))?;
            
            threshold_value = Some(
                threshold_str.trim().parse::<f32>()
                    .map_err(|_| FileUploadError::InvalidThreshold(threshold_str))?
            );
            continue;
        }
        
        // Otherwise, it's a file field
        let filename = field.file_name()
            .ok_or(FileUploadError::MissingFilename)?
            .to_string();

        let data = field.bytes().await
            .map_err(|e| FileUploadError::ReadError(e.to_string()))?
            .to_vec();

        // Check individual file size
        if data.len() > MAX_FILE_SIZE {
            return Err(FileUploadError::FileTooLarge(filename, MAX_FILE_SIZE));
        }

        total_size += data.len();

        // Check total size
        if total_size > MAX_TOTAL_SIZE {
            return Err(FileUploadError::TotalSizeTooLarge(MAX_TOTAL_SIZE));
        }

        files.push((filename, data));

        // Check max files
        if files.len() > MAX_FILES {
            return Err(FileUploadError::TooManyFiles(MAX_FILES));
        }
    }

    // Use provided threshold or default
    let threshold = threshold_value.unwrap_or(DEFAULT_THRESHOLD);
    
    // Validate threshold range
    if threshold < 0.0 || threshold > 1.0 {
        return Err(FileUploadError::InvalidThresholdRange(threshold));
    }

    Ok((files, threshold))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(MAX_FILE_SIZE, 10 * 1024 * 1024);
        assert_eq!(MAX_TOTAL_SIZE, 50 * 1024 * 1024);
        assert_eq!(MAX_FILES, 5);
        assert_eq!(MIN_FILES, 2);
        assert_eq!(DEFAULT_THRESHOLD, 0.70);
    }
}
