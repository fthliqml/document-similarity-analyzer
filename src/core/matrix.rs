//! Similarity Matrix generation - parallel computation

use super::cosine_similarity;
use rayon::prelude::*;

/// Computes a full NxN similarity matrix from TF-IDF vectors.
/// Utilizes parallel processing for efficiency.
///
/// # Arguments
/// * `vectors` - Slice of TF-IDF vectors, one per document
///
/// # Returns
/// NxN matrix where matrix[i][j] = cosine_similarity(vectors[i], vectors[j])
///
/// # Properties
/// - Diagonal is always 1.0 (document is identical to itself)
/// - Matrix is symmetric: matrix[i][j] == matrix[j][i]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_empty_vectors() {
        let vectors: Vec<Vec<f32>> = vec![];
        let matrix = compute_similarity_matrix(&vectors);
        assert!(matrix.is_empty());
    }

    #[test]
    fn test_single_vector() {
        let vectors = vec![vec![1.0, 2.0, 3.0]];
        let matrix = compute_similarity_matrix(&vectors);

        assert_eq!(matrix.len(), 1);
        assert_eq!(matrix[0].len(), 1);
        assert!(approx_eq(matrix[0][0], 1.0));
    }

    #[test]
    fn test_diagonal_is_one() {
        let vectors = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let matrix = compute_similarity_matrix(&vectors);

        for i in 0..vectors.len() {
            assert!(approx_eq(matrix[i][i], 1.0));
        }
    }

    #[test]
    fn test_matrix_symmetry() {
        let vectors = vec![vec![1.0, 0.0], vec![1.0, 1.0], vec![0.0, 1.0]];
        let matrix = compute_similarity_matrix(&vectors);

        for i in 0..vectors.len() {
            for j in 0..vectors.len() {
                assert!(approx_eq(matrix[i][j], matrix[j][i]));
            }
        }
    }

    #[test]
    fn test_identical_vectors_similarity() {
        let vectors = vec![
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 3.0], // identical to first
            vec![4.0, 5.0, 6.0],
        ];
        let matrix = compute_similarity_matrix(&vectors);

        // First two vectors are identical, so similarity should be 1.0
        assert!(approx_eq(matrix[0][1], 1.0));
        assert!(approx_eq(matrix[1][0], 1.0));
    }

    #[test]
    fn test_orthogonal_vectors() {
        let vectors = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let matrix = compute_similarity_matrix(&vectors);

        assert!(approx_eq(matrix[0][1], 0.0));
        assert!(approx_eq(matrix[1][0], 0.0));
    }
}
