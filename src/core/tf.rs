//! Term Frequency calculation - pure function

use std::collections::HashMap;

/// Computes Term Frequency (TF) for a list of tokens.
/// TF = (number of times term appears) / (total number of terms)

pub fn compute_tf(tokens: &[String]) -> HashMap<String, f32> {
    if tokens.is_empty() {
        return HashMap::new();
    }

    let total = tokens.len() as f32;
    
    // Count occurrences using fold (functional approach)
    let counts = tokens.iter().fold(HashMap::new(), |mut acc, token| {
        *acc.entry(token.clone()).or_insert(0) += 1;
        acc
    });

    counts
        .into_iter()
        .map(|(term, count)| (term, count as f32 / total))
        .collect()
}
