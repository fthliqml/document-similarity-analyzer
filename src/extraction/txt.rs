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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_txt_valid() {
        let text = b"Hello, world!\nThis is a test.";
        let result = extract_txt(text);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!\nThis is a test.");
    }

    #[test]
    fn test_extract_txt_empty() {
        let result = extract_txt(&[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_extract_txt_invalid_utf8() {
        let invalid = vec![0xFF, 0xFE, 0xFD];
        let result = extract_txt(&invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_txt_with_whitespace() {
        let text = b"  \n  Hello  \n  ";
        let result = extract_txt(text);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }
}
