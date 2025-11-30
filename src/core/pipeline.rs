//! Document analysis pipeline - parallel processing orchestration

use rayon::prelude::*;
use std::sync::Arc;

use crate::models::SimilarityMatrix;
use super::{
    normalize_text, 
    tokenize, 
    compute_tf, 
    compute_idf, 
    vectorize, 
    compute_similarity_matrix
};

/// Analyzes multiple documents and computes their similarity matrix.
/// Uses parallel processing for all possible stages.
///
/// # Pipeline
/// 1. Normalize all documents (parallel)
/// 2. Tokenize all documents (parallel)
/// 3. Compute TF for each document (parallel)
/// 4. Compute IDF globally (single-threaded, needs all TFs)
/// 5. Build vocabulary from IDF
/// 6. Vectorize each document (parallel)
/// 7. Compute similarity matrix (parallel)
///
/// # Arguments
/// * `documents` - Slice of document strings to analyze
///
/// # Returns
/// A SimilarityMatrix containing the NxN similarity values and document labels
pub fn analyze_documents(documents: &[String]) -> SimilarityMatrix {
    if documents.is_empty() {
        return SimilarityMatrix::new(vec![], vec![]);
    }

    // Generate document labels
    let labels: Vec<String> = (0..documents.len())
        .map(|i| format!("doc{}", i))
        .collect();

    // Stage 1: Normalize (parallel)
    let normalized: Vec<String> = documents
        .par_iter()
        .map(|doc| normalize_text(doc))
        .collect();

    // Stage 2: Tokenize (parallel)
    let tokenized: Vec<Vec<String>> = normalized
        .par_iter()
        .map(|doc| tokenize(doc))
        .collect();

    // Stage 3: Compute TF for each document (parallel)
    let tfs: Vec<_> = tokenized
        .par_iter()
        .map(|tokens| compute_tf(tokens))
        .collect();

    // Stage 4: Compute IDF (single-threaded, needs all TFs)
    let idf = compute_idf(&tfs);

    // Stage 5: Build vocabulary (sorted for consistency)
    let mut vocabulary: Vec<String> = idf.keys().cloned().collect();
    vocabulary.sort();

    // Share vocabulary and IDF across threads
    let vocab_arc = Arc::new(vocabulary);
    let idf_arc = Arc::new(idf);

    // Stage 6: Vectorize each document (parallel)
    let vectors: Vec<Vec<f32>> = tfs
        .par_iter()
        .map(|tf| vectorize(tf, &idf_arc, &vocab_arc))
        .collect();

    // Stage 7: Compute similarity matrix (parallel)
    let matrix = compute_similarity_matrix(&vectors);

    SimilarityMatrix::new(matrix, labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_empty_documents() {
        let docs: Vec<String> = vec![];
        let result = analyze_documents(&docs);
        
        assert!(result.matrix.is_empty());
        assert!(result.index.is_empty());
    }

    #[test]
    fn test_single_document() {
        let docs = vec!["hello world".to_string()];
        let result = analyze_documents(&docs);
        
        assert_eq!(result.matrix.len(), 1);
        assert_eq!(result.index, vec!["doc0"]);
        assert!(approx_eq(result.matrix[0][0], 1.0));
    }

    #[test]
    fn test_identical_documents() {
        let docs = vec![
            "hello world".to_string(),
            "hello world".to_string(),
        ];
        let result = analyze_documents(&docs);
        
        assert_eq!(result.matrix.len(), 2);
        // With smoothed IDF, identical documents should have high similarity
        assert!(approx_eq(result.matrix[0][1], 1.0));
        assert!(approx_eq(result.matrix[1][0], 1.0));
    }

    #[test]
    fn test_different_documents() {
        let docs = vec![
            "the cat sat on the mat".to_string(),
            "the dog ran in the park".to_string(),
            "hello world".to_string(),
        ];
        let result = analyze_documents(&docs);
        
        assert_eq!(result.matrix.len(), 3);
        assert_eq!(result.index.len(), 3);
        
        // All diagonals should be 1.0
        for i in 0..3 {
            assert!(approx_eq(result.matrix[i][i], 1.0));
        }
        
        // First two docs share "the" so should have some similarity
        // Third doc is completely different
        assert!(result.matrix[0][1] > result.matrix[0][2]);
    }

    #[test]
    fn test_deterministic_output() {
        let docs = vec![
            "document one".to_string(),
            "document two".to_string(),
            "something else".to_string(),
        ];
        
        let result1 = analyze_documents(&docs);
        let result2 = analyze_documents(&docs);
        
        // Results should be identical
        for i in 0..result1.matrix.len() {
            for j in 0..result1.matrix[i].len() {
                assert!(approx_eq(result1.matrix[i][j], result2.matrix[i][j]));
            }
        }
    }

    #[test]
    fn test_document_labels() {
        let docs = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let result = analyze_documents(&docs);
        
        assert_eq!(result.index, vec!["doc0", "doc1", "doc2", "doc3"]);
    }
}
