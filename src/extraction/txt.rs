//! TXT text extraction module

/// Extract text from TXT file bytes
///
/// Simply converts bytes to UTF-8 string.
/// Returns the decoded text content.
pub fn extract_txt(file_bytes: &[u8]) -> Result<String, String> {
    String::from_utf8(file_bytes.to_vec())
        .map_err(|e| format!("Failed to decode TXT as UTF-8: {}", e))
        .map(|text| text.trim().to_string())
}
