//! Tokenization - pure function

/// Tokenizes text into a vector of words by splitting on whitespace.

pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
