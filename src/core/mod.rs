//! Core processing functions - all pure functions with no side effects

mod normalize;
mod tokenize;
mod tf;
mod idf;
mod vectorize;
mod similarity;
mod matrix;
mod pipeline;

pub use normalize::normalize_text;
pub use tokenize::tokenize;
pub use tf::compute_tf;
pub use idf::compute_idf;
pub use vectorize::vectorize;
pub use similarity::cosine_similarity;
pub use matrix::compute_similarity_matrix;
pub use pipeline::analyze_documents;
