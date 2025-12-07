//! Cosine Similarity calculation - pure function

use std::collections::HashMap;

/// Computes cosine similarity between two vectors.
/// Formula: (A · B) / (||A|| * ||B||)
pub fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> f32 {
    if vec_a.len() != vec_b.len() || vec_a.is_empty() {
        return 0.0;
    }

    // Compute dot product: A · B
    let dot_product: f32 = vec_a
        .iter()
        .zip(vec_b.iter())
        .map(|(a, b)| a * b)
        .sum();

    // Compute magnitudes: ||A|| and ||B||
    let magnitude_a: f32 = vec_a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = vec_b.iter().map(|x| x * x).sum::<f32>().sqrt();

    // Handle zero magnitude case
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}

/// Compute cosine similarity between two HashMap-based TF-IDF vectors
///
/// # Arguments
/// * `vec_a` - First vector as HashMap
/// * `vec_b` - Second vector as HashMap
///
/// # Returns
/// Cosine similarity value between -1.0 and 1.0
pub fn compute_cosine_similarity(
    vec_a: &HashMap<String, f32>,
    vec_b: &HashMap<String, f32>,
) -> f32 {
    if vec_a.is_empty() || vec_b.is_empty() {
        return 0.0;
    }

    // Compute dot product for common terms
    let dot_product: f32 = vec_a
        .iter()
        .filter_map(|(term, a_val)| vec_b.get(term).map(|b_val| a_val * b_val))
        .sum();

    // Compute magnitudes
    let magnitude_a: f32 = vec_a.values().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = vec_b.values().map(|x| x * x).sum::<f32>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}
