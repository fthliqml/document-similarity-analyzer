//! Cosine Similarity calculation - pure function

/// Computes cosine similarity between two vectors.
/// Formula: (A · B) / (||A|| * ||B||)
///
/// # Arguments
/// * `vec_a` - First vector
/// * `vec_b` - Second vector
///
/// # Returns
/// Cosine similarity value between -1.0 and 1.0
/// Returns 0.0 if either vector has zero magnitude
///
/// # Example
/// ```
/// use document_similarity_analyzer::core::cosine_similarity;
///
/// let a = vec![1.0, 0.0];
/// let b = vec![1.0, 0.0];
/// assert_eq!(cosine_similarity(&a, &b), 1.0);
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_identical_vectors() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(approx_eq(cosine_similarity(&a, &b), 1.0));
    }

    #[test]
    fn test_orthogonal_vectors() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert!(approx_eq(cosine_similarity(&a, &b), 0.0));
    }

    #[test]
    fn test_opposite_vectors() {
        let a = vec![1.0, 2.0];
        let b = vec![-1.0, -2.0];
        assert!(approx_eq(cosine_similarity(&a, &b), -1.0));
    }

    #[test]
    fn test_zero_vector() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(approx_eq(cosine_similarity(&a, &b), 0.0));
    }

    #[test]
    fn test_both_zero_vectors() {
        let a = vec![0.0, 0.0];
        let b = vec![0.0, 0.0];
        assert!(approx_eq(cosine_similarity(&a, &b), 0.0));
    }

    #[test]
    fn test_different_lengths() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(approx_eq(cosine_similarity(&a, &b), 0.0));
    }

    #[test]
    fn test_empty_vectors() {
        let a: Vec<f32> = vec![];
        let b: Vec<f32> = vec![];
        assert!(approx_eq(cosine_similarity(&a, &b), 0.0));
    }

    #[test]
    fn test_similar_vectors() {
        let a = vec![1.0, 1.0];
        let b = vec![1.0, 0.5];
        let similarity = cosine_similarity(&a, &b);
        // Should be high but not 1.0
        assert!(similarity > 0.8);
        assert!(similarity < 1.0);
    }
}
