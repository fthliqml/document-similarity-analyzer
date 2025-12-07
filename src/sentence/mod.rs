//! Sentence splitting module

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex pattern for splitting sentences
    /// Matches punctuation (. ! ?) followed by whitespace (including newlines)
    /// or end of string
    static ref SENTENCE_SPLITTER: Regex = Regex::new(r"[.!?](?:\s+|$)").unwrap();
}

/// Split text into sentences using regex
pub fn split_sentences(text: &str) -> Vec<String> {
    if text.trim().is_empty() {
        return vec![];
    }

    // Collect all matches with their positions
    let matches: Vec<_> = SENTENCE_SPLITTER.find_iter(text).collect();
    
    if matches.is_empty() {
        // No sentence delimiters found, return entire text as one sentence
        return vec![text.trim().to_string()].into_iter()
            .filter(|s| !s.is_empty())
            .collect();
    }

    // Build sentences from matches using functional approach
    let sentence_ranges = matches.iter().enumerate().map(|(idx, mat)| {
        let start = if idx == 0 { 0 } else { matches[idx - 1].end() };
        let end = mat.start() + 1; // +1 to include punctuation
        (start, end)
    });

    // Collect sentences from ranges
    let sentences_from_ranges: Vec<String> = sentence_ranges
        .map(|(start, end)| text[start..end].trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Add remaining text after last match if exists
    let remaining_sentence = matches.last()
        .and_then(|last_match| {
            let remaining = text[last_match.end()..].trim();
            if remaining.is_empty() {
                None
            } else {
                Some(remaining.to_string())
            }
        });

    // Combine sentences with optional remaining
    sentences_from_ranges.into_iter()
        .chain(remaining_sentence)
        .collect()
}
