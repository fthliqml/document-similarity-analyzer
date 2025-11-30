//! Text normalization - pure function

/// Normalizes text by converting to lowercase, removing punctuation,
/// and collapsing multiple whitespace into single space.
///
/// # Arguments
/// * `text` - The input text to normalize
///
/// # Returns
/// A new String with normalized text
///
/// # Example
/// ```
/// use document_similarity_analyzer::core::normalize_text;
///
/// let result = normalize_text("Hello, World!");
/// assert_eq!(result, "hello world");
/// ```
pub fn normalize_text(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_punctuation() {
                ' '
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_conversion() {
        assert_eq!(normalize_text("HELLO"), "hello");
        assert_eq!(normalize_text("HeLLo WoRLD"), "hello world");
    }

    #[test]
    fn test_punctuation_removal() {
        assert_eq!(normalize_text("Hello, World!"), "hello world");
        assert_eq!(normalize_text("test@email.com"), "test email com");
        assert_eq!(normalize_text("don't"), "don t");
    }

    #[test]
    fn test_whitespace_normalization() {
        assert_eq!(normalize_text("  Multiple   Spaces  "), "multiple spaces");
        assert_eq!(normalize_text("\tTabs\nNewlines"), "tabs newlines");
    }

    #[test]
    fn test_mixed_content() {
        assert_eq!(normalize_text("123!@#ABC"), "123 abc");
        assert_eq!(normalize_text("Hello...World???"), "hello world");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(normalize_text(""), "");
    }

    #[test]
    fn test_only_punctuation() {
        assert_eq!(normalize_text("!!!...???"), "");
    }
}
