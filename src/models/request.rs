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
