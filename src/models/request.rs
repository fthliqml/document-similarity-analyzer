//! API request models

use serde::{Deserialize, Serialize};

/// Request payload for document analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    /// List of document texts to analyze
    pub documents: Vec<String>,
}

impl AnalyzeRequest {
    pub fn new(documents: Vec<String>) -> Self {
        Self { documents }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_request_serialization() {
        let request = AnalyzeRequest::new(vec![
            "hello world".to_string(),
            "hello there".to_string(),
        ]);
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: AnalyzeRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.documents.len(), 2);
        assert_eq!(parsed.documents[0], "hello world");
    }
}
