//! Text normalization - pure function

/// Normalizes text by converting to lowercase, removing punctuation,
/// and collapsing multiple whitespace into single space.

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
