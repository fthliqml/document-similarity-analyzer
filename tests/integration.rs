//! Integration tests for Document Similarity Analyzer
//!
//! These tests verify the full pipeline and API endpoints work correctly.

use document_similarity_analyzer::core::analyze_documents;
use document_similarity_analyzer::models::AnalyzeRequest;
use document_similarity_analyzer::api::validate_request;

/// Test full pipeline with known documents
#[test]
fn test_pipeline_with_known_documents() {
    let documents = vec![
        "The quick brown fox jumps over the lazy dog".to_string(),
        "A quick brown dog outpaces a lazy fox".to_string(),
        "Hello world this is a test".to_string(),
    ];

    let result = analyze_documents(&documents);

    // Verify matrix dimensions
    assert_eq!(result.matrix.len(), 3);
    assert_eq!(result.matrix[0].len(), 3);

    // Verify diagonal is 1.0
    for i in 0..3 {
        assert!(
            (result.matrix[i][i] - 1.0).abs() < 0.001,
            "Diagonal element [{i}][{i}] should be 1.0"
        );
    }

    // Verify symmetry
    for i in 0..3 {
        for j in 0..3 {
            assert!(
                (result.matrix[i][j] - result.matrix[j][i]).abs() < 0.001,
                "Matrix should be symmetric at [{i}][{j}]"
            );
        }
    }

    // First two documents should have higher similarity than with the third
    let sim_0_1 = result.matrix[0][1];
    let sim_0_2 = result.matrix[0][2];
    let sim_1_2 = result.matrix[1][2];

    assert!(
        sim_0_1 > sim_0_2,
        "Documents 0 and 1 should be more similar than 0 and 2"
    );
    assert!(
        sim_0_1 > sim_1_2,
        "Documents 0 and 1 should be more similar than 1 and 2"
    );
}

/// Test deterministic output - same input should give same output
#[test]
fn test_deterministic_output() {
    let documents = vec![
        "machine learning is fascinating".to_string(),
        "deep learning neural networks".to_string(),
        "natural language processing".to_string(),
    ];

    let result1 = analyze_documents(&documents);
    let result2 = analyze_documents(&documents);

    // Verify same dimensions
    assert_eq!(result1.matrix.len(), result2.matrix.len());

    // Verify exact same values
    for i in 0..result1.matrix.len() {
        for j in 0..result1.matrix[i].len() {
            assert!(
                (result1.matrix[i][j] - result2.matrix[i][j]).abs() < 0.0001,
                "Results should be deterministic at [{i}][{j}]"
            );
        }
    }
}

/// Test with identical documents
#[test]
fn test_identical_documents() {
    let documents = vec![
        "hello world".to_string(),
        "hello world".to_string(),
    ];

    let result = analyze_documents(&documents);

    // All pairs should be 1.0 for identical documents
    for i in 0..2 {
        for j in 0..2 {
            assert!(
                (result.matrix[i][j] - 1.0).abs() < 0.001,
                "Identical documents should have similarity 1.0 at [{i}][{j}]"
            );
        }
    }
}

/// Test with completely different documents
#[test]
fn test_completely_different_documents() {
    let documents = vec![
        "apple banana cherry".to_string(),
        "xyz uvw rst".to_string(),
    ];

    let result = analyze_documents(&documents);

    // Documents with no common terms should have 0 similarity
    assert!(
        result.matrix[0][1].abs() < 0.001,
        "Documents with no common terms should have ~0 similarity"
    );
}

/// Test with many documents for parallel processing
#[test]
fn test_many_documents_parallel() {
    let documents: Vec<String> = (0..50)
        .map(|i| format!("document number {} with some content here", i))
        .collect();

    let result = analyze_documents(&documents);

    // Verify dimensions
    assert_eq!(result.matrix.len(), 50);
    assert_eq!(result.matrix[0].len(), 50);

    // Verify all values are between 0 and 1
    for i in 0..50 {
        for j in 0..50 {
            let sim = result.matrix[i][j];
            assert!(
                sim >= 0.0 && sim <= 1.0,
                "Similarity at [{i}][{j}] = {sim} should be between 0 and 1"
            );
        }
    }
}

/// Test pipeline with special characters
#[test]
fn test_special_characters() {
    let documents = vec![
        "Hello, World! How are you?".to_string(),
        "HELLO WORLD HOW ARE YOU".to_string(),
    ];

    let result = analyze_documents(&documents);

    // After normalization, these should be identical
    assert!(
        (result.matrix[0][1] - 1.0).abs() < 0.001,
        "Documents that normalize to same text should have similarity 1.0"
    );
}

/// Test with minimum documents (2)
#[test]
fn test_minimum_documents() {
    let documents = vec![
        "first document".to_string(),
        "second document".to_string(),
    ];

    let result = analyze_documents(&documents);

    assert_eq!(result.matrix.len(), 2);
    assert_eq!(result.index.len(), 2);
}

/// Test index labels
#[test]
fn test_index_labels() {
    let documents = vec![
        "doc one".to_string(),
        "doc two".to_string(),
        "doc three".to_string(),
    ];

    let result = analyze_documents(&documents);

    assert_eq!(result.index, vec!["doc0", "doc1", "doc2"]);
}

/// Test empty after normalization
#[test]
fn test_punctuation_only_documents() {
    let documents = vec![
        "hello world".to_string(),
        "!@#$%^&*()".to_string(), // This becomes empty after normalization
    ];

    let result = analyze_documents(&documents);

    // Should still produce a valid matrix
    assert_eq!(result.matrix.len(), 2);
    
    // Empty document should have 0 similarity with everything except itself
    // (depending on implementation, diagonal might still be 1.0 or NaN handled as 0)
}

/// Test with unicode characters
#[test]
fn test_unicode_handling() {
    let documents = vec![
        "café résumé naïve".to_string(),
        "cafe resume naive".to_string(),
    ];

    let result = analyze_documents(&documents);

    // Should produce valid results without panicking
    assert_eq!(result.matrix.len(), 2);
}

/// Test with long documents
#[test]
fn test_long_documents() {
    let long_text: String = (0..1000)
        .map(|i| format!("word{}", i % 100))
        .collect::<Vec<_>>()
        .join(" ");

    let documents = vec![
        long_text.clone(),
        long_text.clone(),
        "completely different text".to_string(),
    ];

    let result = analyze_documents(&documents);

    // Identical long documents should have similarity 1.0
    assert!(
        (result.matrix[0][1] - 1.0).abs() < 0.001,
        "Identical long documents should have similarity 1.0"
    );
}

#[cfg(test)]
mod api_tests {
    use document_similarity_analyzer::models::AnalyzeRequest;
    use document_similarity_analyzer::api::validate_request;

    /// Test request validation - empty documents
    #[test]
    fn test_validate_no_documents() {
        let request = AnalyzeRequest::new(vec![]);
        let result = validate_request(&request);
        assert!(result.is_err());
    }

    /// Test request validation - single document
    #[test]
    fn test_validate_single_document() {
        let request = AnalyzeRequest::new(vec!["only one".to_string()]);
        let result = validate_request(&request);
        assert!(result.is_err());
    }

    /// Test request validation - empty string in documents
    #[test]
    fn test_validate_empty_string() {
        let request = AnalyzeRequest::new(vec!["valid".to_string(), "".to_string()]);
        let result = validate_request(&request);
        assert!(result.is_err());
    }

    /// Test request validation - whitespace only
    #[test]
    fn test_validate_whitespace_only() {
        let request = AnalyzeRequest::new(vec!["valid".to_string(), "   ".to_string()]);
        let result = validate_request(&request);
        assert!(result.is_err());
    }

    /// Test request validation - too many documents
    #[test]
    fn test_validate_too_many_documents() {
        let documents: Vec<String> = (0..101).map(|i| format!("doc {}", i)).collect();
        let request = AnalyzeRequest::new(documents);
        let result = validate_request(&request);
        assert!(result.is_err());
    }

    /// Test request validation - valid request
    #[test]
    fn test_validate_valid_request() {
        let request = AnalyzeRequest::new(vec!["first doc".to_string(), "second doc".to_string()]);
        let result = validate_request(&request);
        assert!(result.is_ok());
    }

    /// Test request validation - exactly 100 documents (boundary)
    #[test]
    fn test_validate_exactly_100_documents() {
        let documents: Vec<String> = (0..100).map(|i| format!("doc {}", i)).collect();
        let request = AnalyzeRequest::new(documents);
        let result = validate_request(&request);
        assert!(result.is_ok());
    }
}
