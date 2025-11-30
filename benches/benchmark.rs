//! Benchmarks for document similarity analyzer

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use document_similarity_analyzer::core::{
    normalize_text, tokenize, compute_tf, analyze_documents
};

fn bench_normalize_text(c: &mut Criterion) {
    let text = "Hello, World! This is a sample document with some punctuation... and numbers 123!";
    
    c.bench_function("normalize_text", |b| {
        b.iter(|| normalize_text(black_box(text)))
    });
}

fn bench_tokenize(c: &mut Criterion) {
    let text = "hello world this is a sample document with multiple words";
    
    c.bench_function("tokenize", |b| {
        b.iter(|| tokenize(black_box(text)))
    });
}

fn bench_compute_tf(c: &mut Criterion) {
    let tokens: Vec<String> = (0..100).map(|i| format!("word{}", i % 20)).collect();
    
    c.bench_function("compute_tf", |b| {
        b.iter(|| compute_tf(black_box(&tokens)))
    });
}

fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline");
    
    for size in [5, 10, 25, 50].iter() {
        let docs: Vec<String> = (0..*size)
            .map(|i| format!(
                "This is document number {} with some sample text for testing the similarity analysis pipeline",
                i
            ))
            .collect();
        
        group.bench_with_input(BenchmarkId::new("docs", size), &docs, |b, docs| {
            b.iter(|| analyze_documents(black_box(docs)))
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_normalize_text,
    bench_tokenize,
    bench_compute_tf,
    bench_full_pipeline
);
criterion_main!(benches);
