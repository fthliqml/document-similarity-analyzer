//! Inverse Document Frequency calculation - pure function

use std::collections::HashMap;

/// Computes Inverse Document Frequency (IDF) across all documents.
/// Uses smoothed IDF: IDF = log((N + 1) / (df + 1)) + 1
pub fn compute_idf(tfs: &[HashMap<String, f32>]) -> HashMap<String, f32> {
    if tfs.is_empty() {
        return HashMap::new();
    }

    let n = tfs.len() as f32;
    
    // Collect all unique terms from all documents and count document frequency
    let document_frequency = tfs
        .iter()
        .flat_map(|tf| tf.keys())
        .fold(HashMap::new(), |mut acc, term| {
            *acc.entry(term.clone()).or_insert(0) += 1;
            acc
        });

    // Calculate smoothed IDF for each term
    // Using: IDF = log((N + 1) / (df + 1)) + 1
    // This ensures IDF is always positive and handles edge cases
    document_frequency
        .into_iter()
        .map(|(term, df)| {
            let idf = ((n + 1.0) / (df as f32 + 1.0)).ln() + 1.0;
            (term, idf)
        })
        .collect()
}
