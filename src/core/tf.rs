//! Term Frequency calculation - pure function

use std::collections::HashMap;

/// Computes Term Frequency (TF) for a list of tokens.
/// TF = (number of times term appears) / (total number of terms)
///
/// # Arguments
/// * `tokens` - Slice of tokens from a document
///
/// # Returns
/// A HashMap mapping each term to its frequency (0.0 to 1.0)
///
/// # Example
/// ```
/// use document_similarity_analyzer::core::compute_tf;
///
/// let tokens = vec!["a".to_string(), "b".to_string(), "a".to_string()];
/// let tf = compute_tf(&tokens);
/// assert!((tf["a"] - 0.667).abs() < 0.01);
/// assert!((tf["b"] - 0.333).abs() < 0.01);
/// ```
pub fn compute_tf(tokens: &[String]) -> HashMap<String, f32> {
    if tokens.is_empty() {
        return HashMap::new();
    }

    let total = tokens.len() as f32;
    let mut counts: HashMap<String, usize> = HashMap::new();

    for token in tokens {
        *counts.entry(token.clone()).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .map(|(term, count)| (term, count as f32 / total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_basic_tf() {
        let tokens = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let tf = compute_tf(&tokens);

        assert!(approx_eq(tf["a"], 2.0 / 3.0));
        assert!(approx_eq(tf["b"], 1.0 / 3.0));
    }

    #[test]
    fn test_single_token() {
        let tokens = vec!["hello".to_string()];
        let tf = compute_tf(&tokens);

        assert_eq!(tf["hello"], 1.0);
    }

    #[test]
    fn test_empty_tokens() {
        let tokens: Vec<String> = vec![];
        let tf = compute_tf(&tokens);

        assert!(tf.is_empty());
    }

    #[test]
    fn test_all_same_tokens() {
        let tokens = vec!["same".to_string(), "same".to_string(), "same".to_string()];
        let tf = compute_tf(&tokens);

        assert_eq!(tf["same"], 1.0);
        assert_eq!(tf.len(), 1);
    }

    #[test]
    fn test_unique_tokens() {
        let tokens = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        let tf = compute_tf(&tokens);

        for (_, freq) in &tf {
            assert!(approx_eq(*freq, 0.25));
        }
    }
}
