//! Manual integration test - demonstrating end-to-end flow

use document_similarity_analyzer::sentence::split_sentences;
use document_similarity_analyzer::core::{analyze_sentence_similarity, SentenceDocument};

#[test]
fn test_end_to_end_workflow() {
    println!("\n🧪 Testing End-to-End Sentence-Level Similarity Workflow");
    println!("{}", "=".repeat(60));
    
    // Step 1: Create sample documents
    println!("\n📄 Step 1: Creating sample documents...");
    
    let doc1_text = "Artificial intelligence is transforming the world. Machine learning is powerful. Neural networks are amazing.";
    let doc2_text = "Artificial intelligence is transforming the world. Deep learning is revolutionary. AI is the future.";
    let doc3_text = "Climate change is a serious issue. Renewable energy is important. Solar power is growing.";
    
    // Step 2: Split into sentences
    println!("✂️  Step 2: Splitting texts into sentences...");
    
    let doc1_sentences = split_sentences(doc1_text);
    let doc2_sentences = split_sentences(doc2_text);
    let doc3_sentences = split_sentences(doc3_text);
    
    println!("  • Doc1: {} sentences", doc1_sentences.len());
    println!("  • Doc2: {} sentences", doc2_sentences.len());
    println!("  • Doc3: {} sentences", doc3_sentences.len());
    
    for (i, sent) in doc1_sentences.iter().enumerate() {
        println!("    - Doc1[{}]: {}", i, sent);
    }
    
    // Step 3: Create SentenceDocuments
    println!("\n📦 Step 3: Creating SentenceDocument objects...");
    
    let documents = vec![
        SentenceDocument::new("doc1.txt".to_string(), doc1_sentences),
        SentenceDocument::new("doc2.txt".to_string(), doc2_sentences),
        SentenceDocument::new("doc3.txt".to_string(), doc3_sentences),
    ];
    
    println!("  ✓ Created {} documents", documents.len());
    
    // Step 4: Analyze similarity
    println!("\n🔍 Step 4: Analyzing sentence-level similarity...");
    
    let threshold = 0.70;
    let (matches, global_similarity) = analyze_sentence_similarity(&documents, threshold);
    
    println!("  ✓ Found {} matches above threshold {}", matches.len(), threshold);
    println!("  ✓ Computed {} global similarity scores", global_similarity.len());
    
    // Step 5: Display results
    println!("\n📊 Step 5: Results");
    println!("{}", "-".repeat(60));
    
    if !matches.is_empty() {
        println!("\n🎯 Sentence Matches (threshold ≥ {}):", threshold);
        for (i, m) in matches.iter().enumerate().take(5) {
            println!("\n  Match #{}:", i + 1);
            println!("    Source: {} [sentence {}]", m.source_doc, m.source_sentence_index);
            println!("    Target: {} [sentence {}]", m.target_doc, m.target_sentence_index);
            println!("    Similarity: {:.4}", m.similarity);
        }
    } else {
        println!("\n⚠️  No matches found above threshold {}", threshold);
    }
    
    if !global_similarity.is_empty() {
        println!("\n🌍 Global Document Similarities:");
        for sim in &global_similarity {
            println!("  • {} <-> {}: {:.4}", sim.doc_a, sim.doc_b, sim.score);
        }
    }
    
    // Step 6: Assertions
    println!("\n✅ Step 6: Validating results...");
    
    assert!(documents.len() == 3, "Should have 3 documents");
    assert!(matches.len() > 0, "Should find at least some matches");
    assert!(global_similarity.len() == 3, "Should have 3 pairwise similarities (n*(n-1)/2)");
    
    // Verify that doc1 and doc2 have higher similarity than doc1 and doc3
    let doc1_doc2_sim = global_similarity.iter()
        .find(|s| (s.doc_a == "doc1.txt" && s.doc_b == "doc2.txt") || 
                   (s.doc_a == "doc2.txt" && s.doc_b == "doc1.txt"))
        .expect("Should find doc1-doc2 similarity");
    
    let doc1_doc3_sim = global_similarity.iter()
        .find(|s| (s.doc_a == "doc1.txt" && s.doc_b == "doc3.txt") || 
                   (s.doc_a == "doc3.txt" && s.doc_b == "doc1.txt"))
        .expect("Should find doc1-doc3 similarity");
    
    println!("  ✓ Doc1-Doc2 similarity: {:.4}", doc1_doc2_sim.score);
    println!("  ✓ Doc1-Doc3 similarity: {:.4}", doc1_doc3_sim.score);
    
    assert!(
        doc1_doc2_sim.score > doc1_doc3_sim.score,
        "Doc1 and Doc2 (AI-related) should be more similar than Doc1 and Doc3 (different topics)"
    );
    
    println!("  ✓ Similarity ranking is correct!");
    
    println!("\n{}", "=".repeat(60));
    println!("🎉 ALL TESTS PASSED!");
    println!("{}", "=".repeat(60));
}
