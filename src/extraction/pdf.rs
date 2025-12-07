//! PDF text extraction module

/// Extract text from PDF file bytes
///
/// Uses pdf-extract library to parse PDF and extract text content.
/// Returns concatenated text from all pages.
pub fn extract_pdf(file_bytes: &[u8]) -> Result<String, String> {
    pdf_extract::extract_text_from_mem(file_bytes)
        .map_err(|e| format!("Failed to extract PDF: {}", e))
        .map(|text| text.trim().to_string())
}
