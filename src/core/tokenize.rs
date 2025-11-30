//! Tokenization - pure function

/// Tokenizes text into a vector of words by splitting on whitespace.
///
/// # Arguments
/// * `text` - The input text to tokenize (should be pre-normalized)
///
/// # Returns
/// A Vec<String> containing individual tokens
///
/// # Example
/// ```
/// use document_similarity_analyzer::core::tokenize;
///
/// let tokens = tokenize("hello world");
/// assert_eq!(tokens, vec!["hello", "world"]);
/// ```
pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        assert_eq!(tokenize("hello world"), vec!["hello", "world"]);
    }

    #[test]
    fn test_single_word() {
        assert_eq!(tokenize("one"), vec!["one"]);
    }

    #[test]
    fn test_empty_string() {
        let result: Vec<String> = tokenize("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(tokenize("hello    world"), vec!["hello", "world"]);
    }

    #[test]
    fn test_tabs_and_newlines() {
        assert_eq!(tokenize("hello\tworld\ntest"), vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_only_whitespace() {
        let result: Vec<String> = tokenize("   \t\n   ");
        assert!(result.is_empty());
    }
}
