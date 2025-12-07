//! Sentence-level document similarity analysis pipeline

use std::collections::HashMap;
use rayon::prelude::*;

use crate::core::{compute_tf, compute_idf, normalize_text, tokenize, compute_tfidf_vector, compute_cosine_similarity};
use crate::models::{SentenceMatch, GlobalSimilarity};

/// Represents a document with its sentences
#[derive(Debug, Clone)]
pub struct SentenceDocument {
    pub filename: String,
    pub sentences: Vec<String>,
}

impl SentenceDocument {
    pub fn new(filename: String, sentences: Vec<String>) -> Self {
        Self { filename, sentences }
    }
}

/// Represents a sentence with its TF-IDF vector
#[derive(Debug, Clone)]
struct SentenceVector {
    doc_index: usize,
    sentence_index: usize,
    vector: HashMap<String, f32>,
}

/// Analyze sentence-level similarity across multiple documents
pub fn analyze_sentence_similarity(
    documents: &[SentenceDocument],
    threshold: f32,
) -> (Vec<SentenceMatch>, Vec<GlobalSimilarity>) {
    // Step 1: Flatten all sentences with their document context
    let all_sentences: Vec<(usize, usize, String)> = documents
        .iter()
        .enumerate()
        .flat_map(|(doc_idx, doc)| {
            doc.sentences
                .iter()
                .enumerate()
                .map(move |(sent_idx, sentence)| (doc_idx, sent_idx, sentence.clone()))
        })
        .collect();

    if all_sentences.is_empty() {
        return (vec![], vec![]);
    }

    // Step 2: Process each sentence (normalize + tokenize)
    let processed_sentences: Vec<(usize, usize, String, Vec<String>)> = all_sentences
        .par_iter()
        .map(|(doc_idx, sent_idx, text)| {
            let normalized = normalize_text(text);
            let tokens = tokenize(&normalized);
            (*doc_idx, *sent_idx, text.clone(), tokens)
        })
        .collect();

    // Step 3: Compute TF for each sentence
    let sentence_tfs: Vec<(usize, usize, String, HashMap<String, f32>)> = processed_sentences
        .into_par_iter()
        .map(|(doc_idx, sent_idx, text, tokens)| {
            let tf = compute_tf(&tokens);
            (doc_idx, sent_idx, text, tf)
        })
        .collect();

    // Step 4: Compute global IDF from all sentences
    let tfs_only: Vec<HashMap<String, f32>> = sentence_tfs
        .iter()
        .map(|(_, _, _, tf)| tf.clone())
        .collect();
    let global_idf = compute_idf(&tfs_only);

    // Step 5: Compute TF-IDF vectors for each sentence
    let sentence_vectors: Vec<SentenceVector> = sentence_tfs
        .into_par_iter()
        .map(|(doc_idx, sent_idx, _text, tf)| {
            let vector = compute_tfidf_vector(&tf, &global_idf);
            SentenceVector {
                doc_index: doc_idx,
                sentence_index: sent_idx,
                vector,
            }
        })
        .collect();

    // Step 6: Compute pairwise similarities (cross-document only)
    let matches = compute_sentence_matches(&sentence_vectors, documents, threshold);

    // Step 7: Compute global document similarities
    let global_similarities = compute_global_similarities(&sentence_vectors, documents);

    (matches, global_similarities)
}

fn compute_sentence_matches(
    vectors: &[SentenceVector],
    documents: &[SentenceDocument],
    threshold: f32,
) -> Vec<SentenceMatch> {
    // Generate all pairs, filter by threshold, and sort by similarity descending
    let mut matches: Vec<SentenceMatch> = vectors
        .iter()
        .enumerate()
        .flat_map(|(i, vec_a)| {
            vectors.iter().skip(i + 1).filter_map(move |vec_b| {
                // Only compare sentences from different documents
                if vec_a.doc_index == vec_b.doc_index {
                    return None;
                }

                let similarity = compute_cosine_similarity(&vec_a.vector, &vec_b.vector);

                if similarity >= threshold {
                    let source_doc = documents[vec_a.doc_index].filename.clone();
                    let target_doc = documents[vec_b.doc_index].filename.clone();
                    
                    // Get actual sentence text
                    let source_sentence = documents[vec_a.doc_index].sentences[vec_a.sentence_index].clone();
                    let target_sentence = documents[vec_b.doc_index].sentences[vec_b.sentence_index].clone();

                    Some(SentenceMatch::new(
                        source_doc,
                        vec_a.sentence_index,
                        source_sentence,
                        target_doc,
                        vec_b.sentence_index,
                        target_sentence,
                        similarity,
                    ))
                } else {
                    None
                }
            })
        })
        .collect();

    // Sort by similarity descending (must use mut here as sort_by requires &mut self)
    matches.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

    matches
}

/// Compute global similarity between document pairs
fn compute_global_similarities(
    vectors: &[SentenceVector],
    documents: &[SentenceDocument],
) -> Vec<GlobalSimilarity> {
    // Group vectors by document using fold (more functional than mut + for loop)
    let doc_vectors: HashMap<usize, Vec<&SentenceVector>> = vectors
        .iter()
        .fold(HashMap::new(), |mut acc, vector| {
            acc.entry(vector.doc_index)
                .or_insert_with(Vec::new)
                .push(vector);
            acc
        });

    let empty_vec: Vec<&SentenceVector> = Vec::new();

    // Collect all document pairs first, then map to compute similarities
    let doc_pairs: Vec<(usize, usize)> = (0..documents.len())
        .flat_map(|doc_a_idx| {
            ((doc_a_idx + 1)..documents.len()).map(move |doc_b_idx| (doc_a_idx, doc_b_idx))
        })
        .collect();

    // Compute similarity for each pair
    let mut global_sims: Vec<GlobalSimilarity> = doc_pairs
        .iter()
        .filter_map(|(doc_a_idx, doc_b_idx)| {
            let vecs_a = doc_vectors.get(doc_a_idx).unwrap_or(&empty_vec);
            let vecs_b = doc_vectors.get(doc_b_idx).unwrap_or(&empty_vec);

            if vecs_a.is_empty() || vecs_b.is_empty() {
                return None;
            }

            // Compute all cross-document sentence similarities using flat_map
            let similarities: Vec<f32> = vecs_a
                .iter()
                .flat_map(|vec_a| {
                    vecs_b.iter().map(|vec_b| {
                        compute_cosine_similarity(&vec_a.vector, &vec_b.vector)
                    })
                })
                .collect();

            // Average similarity
            let avg_similarity = similarities.iter().sum::<f32>() / similarities.len() as f32;

            Some(GlobalSimilarity::new(
                documents[*doc_a_idx].filename.clone(),
                documents[*doc_b_idx].filename.clone(),
                avg_similarity,
            ))
        })
        .collect();

    // Sort by score descending (must use mut here as sort_by requires &mut self)
    global_sims.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    global_sims
}
