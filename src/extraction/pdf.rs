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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pdf_empty_bytes() {
        let result = extract_pdf(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_pdf_invalid_bytes() {
        let invalid_pdf = b"This is not a valid PDF file";
        let result = extract_pdf(invalid_pdf);
        assert!(result.is_err());
    }
}
