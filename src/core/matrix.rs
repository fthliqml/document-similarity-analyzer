//! Similarity Matrix generation - parallel computation

use super::cosine_similarity;
use rayon::prelude::*;

pub fn compute_similarity_matrix(vectors: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let n = vectors.len(); //TODO penamaan variabel

    if n == 0 {
        return vec![];
    }

    // Parallel computation of similarity matrix
    (0..n)
        .into_par_iter()
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        1.0 // Diagonal is always 1.0
                    } else if i < j {
                        // Compute similarity for upper triangle
                        cosine_similarity(&vectors[i], &vectors[j])
                    } else {
                        // For lower triangle, we'll compute it too
                        // (could optimize by computing upper only and copying)
                        cosine_similarity(&vectors[i], &vectors[j])
                    }
                })
                .collect()
        })
        .collect()
}
