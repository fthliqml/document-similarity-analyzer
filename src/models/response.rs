//! API response models

use serde::{Deserialize, Serialize};

/// Response payload for document analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    /// NxN similarity matrix
    pub similarity_matrix: Vec<Vec<f32>>,
    /// Document indices/labels
    pub index: Vec<String>,
}

impl AnalyzeResponse {
    pub fn new(similarity_matrix: Vec<Vec<f32>>, index: Vec<String>) -> Self {
        Self {
            similarity_matrix,
            index,
        }
    }
}

impl From<crate::models::SimilarityMatrix> for AnalyzeResponse {
    fn from(matrix: crate::models::SimilarityMatrix) -> Self {
        Self {
            similarity_matrix: matrix.matrix,
            index: matrix.index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_response_serialization() {
        let response = AnalyzeResponse::new(
            vec![vec![1.0, 0.5], vec![0.5, 1.0]],
            vec!["doc0".to_string(), "doc1".to_string()],
        );
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("similarity_matrix"));
        assert!(json.contains("index"));
    }
}
