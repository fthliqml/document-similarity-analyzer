//! Inverse Document Frequency calculation - pure function

use std::collections::{HashMap, HashSet};

/// Computes Inverse Document Frequency (IDF) across all documents.
/// Uses smoothed IDF: IDF = log((N + 1) / (df + 1)) + 1
/// This prevents division by zero and ensures non-zero IDF values.
///
/// # Arguments
/// * `tfs` - Slice of Term Frequency maps, one per document
///
/// # Returns
/// A HashMap mapping each term to its IDF value
///
/// # Example
/// ```
/// use document_similarity_analyzer::core::compute_idf;
/// use std::collections::HashMap;
///
/// let tf1: HashMap<String, f32> = [("hello".to_string(), 0.5)].into_iter().collect();
/// let tf2: HashMap<String, f32> = [("world".to_string(), 0.5)].into_iter().collect();
/// let idf = compute_idf(&[tf1, tf2]);
/// ```
pub fn compute_idf(tfs: &[HashMap<String, f32>]) -> HashMap<String, f32> {
    if tfs.is_empty() {
        return HashMap::new();
    }

    let n = tfs.len() as f32;
    let mut document_frequency: HashMap<String, usize> = HashMap::new();

    // Count in how many documents each term appears
    for tf in tfs {
        let terms: HashSet<&String> = tf.keys().collect();
        for term in terms {
            *document_frequency.entry(term.clone()).or_insert(0) += 1;
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_single_document() {
        let tf: HashMap<String, f32> = [
            ("hello".to_string(), 0.5),
            ("world".to_string(), 0.5),
        ].into_iter().collect();
        
        let idf = compute_idf(&[tf]);

        // With smoothed IDF: log((1+1)/(1+1)) + 1 = log(1) + 1 = 1.0
        assert!(approx_eq(idf["hello"], 1.0));
        assert!(approx_eq(idf["world"], 1.0));
    }

    #[test]
    fn test_multiple_documents() {
        let tf1: HashMap<String, f32> = [
            ("hello".to_string(), 0.5),
            ("world".to_string(), 0.5),
        ].into_iter().collect();
        
        let tf2: HashMap<String, f32> = [
            ("hello".to_string(), 1.0),
        ].into_iter().collect();

        let idf = compute_idf(&[tf1, tf2]);

        // "hello" appears in 2/2 docs: IDF = log((2+1)/(2+1)) + 1 = 1.0
        assert!(approx_eq(idf["hello"], 1.0));
        // "world" appears in 1/2 docs: IDF = log((2+1)/(1+1)) + 1 = log(1.5) + 1
        let expected_world = (3.0_f32 / 2.0).ln() + 1.0;
        assert!(approx_eq(idf["world"], expected_world));
    }

    #[test]
    fn test_empty_tfs() {
        let tfs: Vec<HashMap<String, f32>> = vec![];
        let idf = compute_idf(&tfs);
        assert!(idf.is_empty());
    }

    #[test]
    fn test_rare_term_high_idf() {
        let tf1: HashMap<String, f32> = [("common".to_string(), 1.0)].into_iter().collect();
        let tf2: HashMap<String, f32> = [("common".to_string(), 1.0)].into_iter().collect();
        let tf3: HashMap<String, f32> = [
            ("common".to_string(), 0.5),
            ("rare".to_string(), 0.5),
        ].into_iter().collect();

        let idf = compute_idf(&[tf1, tf2, tf3]);

        // "rare" should have higher IDF than "common"
        assert!(idf["rare"] > idf["common"]);
    }

    #[test]
    fn test_idf_always_positive() {
        // Even when all docs have the same term, IDF should be positive
        let tf1: HashMap<String, f32> = [("common".to_string(), 1.0)].into_iter().collect();
        let tf2: HashMap<String, f32> = [("common".to_string(), 1.0)].into_iter().collect();
        
        let idf = compute_idf(&[tf1, tf2]);
        
        assert!(idf["common"] > 0.0);
    }
}
