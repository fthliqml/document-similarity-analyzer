//! TF-IDF Vectorization - pure function

use std::collections::HashMap;

/// Converts TF and IDF into a TF-IDF vector based on vocabulary order.
///
/// # Arguments
/// * `tf` - Term Frequency map for a document
/// * `idf` - Inverse Document Frequency map (global)
/// * `vocabulary` - Ordered list of all terms (defines vector dimensions)
///
/// # Returns
/// A Vec<f32> representing the document's TF-IDF vector
///
/// # Example
/// ```
/// use document_similarity_analyzer::core::vectorize;
/// use std::collections::HashMap;
///
/// let tf: HashMap<String, f32> = [("hello".to_string(), 0.5)].into_iter().collect();
/// let idf: HashMap<String, f32> = [("hello".to_string(), 0.693)].into_iter().collect();
/// let vocab = vec!["hello".to_string()];
/// let vector = vectorize(&tf, &idf, &vocab);
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_basic_vectorization() {
        let tf: HashMap<String, f32> = [
            ("hello".to_string(), 0.5),
            ("world".to_string(), 0.5),
        ].into_iter().collect();
        
        let idf: HashMap<String, f32> = [
            ("hello".to_string(), 1.0),
            ("world".to_string(), 2.0),
        ].into_iter().collect();
        
        let vocab = vec!["hello".to_string(), "world".to_string()];
        let vector = vectorize(&tf, &idf, &vocab);

        assert_eq!(vector.len(), 2);
        assert!(approx_eq(vector[0], 0.5)); // 0.5 * 1.0
        assert!(approx_eq(vector[1], 1.0)); // 0.5 * 2.0
    }

    #[test]
    fn test_missing_term_in_tf() {
        let tf: HashMap<String, f32> = [
            ("hello".to_string(), 1.0),
        ].into_iter().collect();
        
        let idf: HashMap<String, f32> = [
            ("hello".to_string(), 1.0),
            ("world".to_string(), 2.0),
        ].into_iter().collect();
        
        let vocab = vec!["hello".to_string(), "world".to_string()];
        let vector = vectorize(&tf, &idf, &vocab);

        assert!(approx_eq(vector[0], 1.0));
        assert!(approx_eq(vector[1], 0.0)); // missing term = 0
    }

    #[test]
    fn test_empty_vocabulary() {
        let tf: HashMap<String, f32> = HashMap::new();
        let idf: HashMap<String, f32> = HashMap::new();
        let vocab: Vec<String> = vec![];
        let vector = vectorize(&tf, &idf, &vocab);

        assert!(vector.is_empty());
    }

    #[test]
    fn test_vocabulary_order_matters() {
        let tf: HashMap<String, f32> = [
            ("a".to_string(), 0.3),
            ("b".to_string(), 0.7),
        ].into_iter().collect();
        
        let idf: HashMap<String, f32> = [
            ("a".to_string(), 1.0),
            ("b".to_string(), 1.0),
        ].into_iter().collect();
        
        let vocab1 = vec!["a".to_string(), "b".to_string()];
        let vocab2 = vec!["b".to_string(), "a".to_string()];
        
        let vector1 = vectorize(&tf, &idf, &vocab1);
        let vector2 = vectorize(&tf, &idf, &vocab2);

        assert!(approx_eq(vector1[0], 0.3));
        assert!(approx_eq(vector1[1], 0.7));
        assert!(approx_eq(vector2[0], 0.7));
        assert!(approx_eq(vector2[1], 0.3));
    }
}
