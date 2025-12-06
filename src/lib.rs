//! Document Similarity Analyzer
//!
//! A backend service for analyzing document similarity using TF-IDF and Cosine Similarity
//! with parallel processing powered by Rayon.
//!
//! ## Architecture
//! - `api` - HTTP API handlers and server configuration
//! - `core` - Pure functions for text processing and similarity computation
//! - `models` - Immutable data structures
//! - `extraction` - File extraction modules (PDF, DOCX, TXT)
//! - `sentence` - Sentence splitting utilities

pub mod api;
pub mod core;
pub mod models;
pub mod extraction;
pub mod sentence;
