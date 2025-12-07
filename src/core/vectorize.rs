//! TF-IDF Vectorization - pure function

use std::collections::HashMap;

/// Converts TF and IDF into a TF-IDF vector based on vocabulary order.
pub fn vectorize(
    tf: &HashMap<String, f32>,
    idf: &HashMap<String, f32>,
    vocabulary: &[String],
) -> Vec<f32> {
    vocabulary
        .iter()
        .map(|term| {
            let tf_value = tf.get(term).copied().unwrap_or(0.0);
            let idf_value = idf.get(term).copied().unwrap_or(0.0);
            tf_value * idf_value
        })
        .collect()
}

/// Compute TF-IDF vector directly as HashMap (for sentence-level analysis)
pub fn compute_tfidf_vector(
    tf: &HashMap<String, f32>,
    idf: &HashMap<String, f32>,
) -> HashMap<String, f32> {
    tf.iter()
        .map(|(term, tf_value)| {
            let idf_value = idf.get(term).copied().unwrap_or(0.0);
            (term.clone(), tf_value * idf_value)
        })
        .collect()
}
