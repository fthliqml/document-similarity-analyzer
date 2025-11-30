//! Document and processing data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a raw document with its ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub content: String,
}

impl Document {
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
        }
    }
}

/// Represents a tokenized document
#[derive(Debug, Clone)]
pub struct TokenizedDoc {
    pub id: String,
    pub tokens: Vec<String>,
}

impl TokenizedDoc {
    pub fn new(id: impl Into<String>, tokens: Vec<String>) -> Self {
        Self {
            id: id.into(),
            tokens,
        }
    }
}

/// Term Frequency map for a single document
pub type TermFrequency = HashMap<String, f32>;

/// Inverse Document Frequency map across all documents
pub type InverseDocumentFrequency = HashMap<String, f32>;

/// TF-IDF Vector representation of a document
#[derive(Debug, Clone)]
pub struct TfIdfVector {
    pub id: String,
    pub vector: Vec<f32>,
}

impl TfIdfVector {
    pub fn new(id: impl Into<String>, vector: Vec<f32>) -> Self {
        Self {
            id: id.into(),
            vector,
        }
    }
}

/// Similarity matrix result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityMatrix {
    /// NxN similarity matrix where matrix[i][j] is similarity between doc i and doc j
    pub matrix: Vec<Vec<f32>>,
    /// Document indices/labels
    pub index: Vec<String>,
}

impl SimilarityMatrix {
    pub fn new(matrix: Vec<Vec<f32>>, index: Vec<String>) -> Self {
        Self { matrix, index }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("doc1", "Hello world");
        assert_eq!(doc.id, "doc1");
        assert_eq!(doc.content, "Hello world");
    }

    #[test]
    fn test_similarity_matrix_creation() {
        let matrix = vec![
            vec![1.0, 0.5],
            vec![0.5, 1.0],
        ];
        let index = vec!["doc0".to_string(), "doc1".to_string()];
        let sim = SimilarityMatrix::new(matrix.clone(), index.clone());
        
        assert_eq!(sim.matrix, matrix);
        assert_eq!(sim.index, index);
    }
}
