//! Models for sentence-level document analysis

use serde::{Deserialize, Serialize};

/// Metadata for analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    /// Total number of documents analyzed
    pub documents_count: usize,
    /// Total number of sentences across all documents
    pub total_sentences: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Similarity threshold used for filtering
    pub threshold: f32,
}

impl AnalysisMetadata {
    pub fn new(
        documents_count: usize,
        total_sentences: usize,
        processing_time_ms: u64,
        threshold: f32,
    ) -> Self {
        Self {
            documents_count,
            total_sentences,
            processing_time_ms,
            threshold,
        }
    }
}

/// A single sentence similarity match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceMatch {
    /// Source document filename
    pub source_doc: String,
    /// Index of sentence in source document (0-based)
    pub source_sentence_index: usize,
    /// The actual source sentence text
    pub source_sentence: String,
    /// Target document filename
    pub target_doc: String,
    /// Index of sentence in target document (0-based)
    pub target_sentence_index: usize,
    /// The actual target sentence text
    pub target_sentence: String,
    /// Similarity score (0.0 to 1.0)
    pub similarity: f32,
}

impl SentenceMatch {
    pub fn new(
        source_doc: String,
        source_sentence_index: usize,
        source_sentence: String,
        target_doc: String,
        target_sentence_index: usize,
        target_sentence: String,
        similarity: f32,
    ) -> Self {
        Self {
            source_doc,
            source_sentence_index,
            source_sentence,
            target_doc,
            target_sentence_index,
            target_sentence,
            similarity,
        }
    }
}

/// Global similarity between two documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSimilarity {
    /// First document filename
    #[serde(rename = "docA")]
    pub doc_a: String,
    /// Second document filename
    #[serde(rename = "docB")]
    pub doc_b: String,
    /// Overall similarity score (0.0 to 1.0)
    pub score: f32,
}

impl GlobalSimilarity {
    pub fn new(doc_a: String, doc_b: String, score: f32) -> Self {
        Self { doc_a, doc_b, score }
    }
}

/// Response payload for sentence-level document analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceAnalysisResponse {
    /// Analysis metadata
    pub metadata: AnalysisMetadata,
    /// List of sentence matches above threshold
    pub matches: Vec<SentenceMatch>,
    /// Global similarity scores between document pairs
    pub global_similarity: Vec<GlobalSimilarity>,
}

impl SentenceAnalysisResponse {
    pub fn new(
        metadata: AnalysisMetadata,
        matches: Vec<SentenceMatch>,
        global_similarity: Vec<GlobalSimilarity>,
    ) -> Self {
        Self {
            metadata,
            matches,
            global_similarity,
        }
    }
}
