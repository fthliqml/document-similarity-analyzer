//! Core processing functions - all pure functions with no side effects

mod normalize;
mod tokenize;
mod tf;
mod idf;
mod vectorize;
mod similarity;
mod matrix;
mod pipeline;
mod sentence_pipeline;

pub use normalize::normalize_text;
pub use tokenize::tokenize;
pub use tf::compute_tf;
pub use idf::compute_idf;
pub use vectorize::{vectorize, compute_tfidf_vector};
pub use similarity::{cosine_similarity, compute_cosine_similarity};
pub use matrix::compute_similarity_matrix;
pub use pipeline::analyze_documents;
pub use sentence_pipeline::{analyze_sentence_similarity, SentenceDocument};
