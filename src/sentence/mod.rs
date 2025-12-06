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
///
/// Uses regex pattern: `[.!?](?:\s+|$)`
/// This splits on punctuation marks (., !, ?) followed by whitespace or end of string.
///
/// # Arguments
/// * `text` - The input text to split
///
/// # Returns
/// Vector of sentence strings, trimmed and non-empty
///
/// # Example
/// ```
/// use document_similarity_analyzer::sentence::split_sentences;
///
/// let text = "Hello world. How are you? I am fine!";
/// let sentences = split_sentences(text);
/// assert_eq!(sentences.len(), 3);
/// assert_eq!(sentences[0], "Hello world.");
/// ```
pub fn split_sentences(text: &str) -> Vec<String> {
    if text.trim().is_empty() {
        return vec![];
    }

    // Find all sentence boundaries
    let mut sentences = Vec::new();
    let mut last_end = 0;

    for mat in SENTENCE_SPLITTER.find_iter(text) {
        // Include the punctuation mark in the sentence
        let sentence_end = mat.start() + 1; // +1 to include the punctuation
        let sentence = text[last_end..sentence_end].trim();
        
        if !sentence.is_empty() {
            sentences.push(sentence.to_string());
        }
        
        last_end = mat.end(); // Skip the whitespace
    }

    // Add the remaining text as the last sentence (if any)
    if last_end < text.len() {
        let last_sentence = text[last_end..].trim();
        if !last_sentence.is_empty() {
            sentences.push(last_sentence.to_string());
        }
    }

    sentences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_sentences_basic() {
        let text = "Hello world. How are you? I am fine!";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "Hello world.");
        assert_eq!(sentences[1], "How are you?");
        assert_eq!(sentences[2], "I am fine!");
    }

    #[test]
    fn test_split_sentences_single() {
        let text = "This is a single sentence.";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 1);
        assert_eq!(sentences[0], "This is a single sentence.");
    }

    #[test]
    fn test_split_sentences_no_punctuation() {
        let text = "No punctuation here";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 1);
        assert_eq!(sentences[0], "No punctuation here");
    }

    #[test]
    fn test_split_sentences_empty() {
        let text = "";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 0);
    }

    #[test]
    fn test_split_sentences_whitespace_only() {
        let text = "   \n  \t  ";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 0);
    }

    #[test]
    fn test_split_sentences_multiple_spaces() {
        let text = "First sentence.    Second sentence!     Third.";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "First sentence.");
        assert_eq!(sentences[1], "Second sentence!");
        assert_eq!(sentences[2], "Third.");
    }

    #[test]
    fn test_split_sentences_with_abbreviations() {
        // Note: regex doesn't handle abbreviations specially
        // "Tn. Budi" will be split, but that's acceptable per requirements
        let text = "Tn. Budi pergi. Dia senang.";
        let sentences = split_sentences(text);
        // Will split after "Tn." - acceptable trade-off
        assert!(sentences.len() >= 2);
    }

    #[test]
    fn test_split_sentences_newlines() {
        let text = "First line.\nSecond line.\n\nThird line.";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 3);
    }
}
